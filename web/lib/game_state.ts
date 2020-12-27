import { Coordinates, CPUTurnUpdate, EndTurnUpdate, Rotation, Tile, TilePath, TilePathType, TilePlacementEvent, tile_path_to_tile, TurnScore, WasmNile } from "nile";
import React from "react";
import { BoardArray, Cell, CoordinateTuple, PlayerData, sumTurnScores, TilePlacement, toBoardArray, toPlayerDataArray } from "./common";
import { maxBy, mod } from "./utils";
import { CoordinateSet } from "./CoordinateSet";

interface IRackDraggedTile {
    idx: number;
    tilePath: TilePath;
    isUniversal: boolean;
}

type SelectedTile =
    | {type: "rack", tile: IRackDraggedTile}
    | {type: "board", coordinates: CoordinateTuple};

type Modal =
    | {type: "error", msg: string}
    | {type: "endOfGame", msg: string};

interface IInnerState {
    board: BoardArray;
    currentPlayerId: number;
    /** Whether the game has finished */
    gameHasEnded: boolean;
    playerData: PlayerData[];
    /** Used for determining if placed tile is movable, rotatable, etc. */
    currentTurnTiles: CoordinateSet;
    selectedTile: SelectedTile | null;
    modal: Modal | null;
}

interface IState {
    past: IInnerState[];
    now: IInnerState;
    future: IInnerState[];
}

type Action =
    | {type: "selectRackTile", tilePath: TilePath, isUniversal: boolean, idx: number}
    | {type: "selectBoardTile", coordinates: CoordinateTuple}
    | {type: "placeTile", draggedTile: IRackDraggedTile, coordinates: CoordinateTuple, rotation: Rotation, score: TurnScore}
    | {type: "rotateTile", coordinates: CoordinateTuple, rotation: Rotation}
    | {type: "removeTile", coordinates: CoordinateTuple, score: TurnScore}
    | {type: "updateUniversalPath", coordinates: CoordinateTuple, tilePlacement: TilePlacement}
    | {type: "moveTile", oldCoordinates: CoordinateTuple, newCoordinates: CoordinateTuple, tilePlacement: TilePlacement, score: TurnScore}
    | {type: "undo"}
    | {type: "redo"}
    /** Same event for cantPlay */
    | {type: "endTurn", endTurnUpdate: EndTurnUpdate}
    | {type: "cpuTurn", cpuUpdate: CPUTurnUpdate}
    | {type: "setError", err: unknown}
    | {type: "setEndOfGame", msg: string}
    | {type: "dismiss"}

export const initState = (nile: WasmNile): IState => {
    const board = nile.board();
    return {
        past: [],
        now: {
            board: toBoardArray(board),
            currentPlayerId: 0,
            gameHasEnded: false,
            playerData: toPlayerDataArray(nile.players()),
            currentTurnTiles: new CoordinateSet(board.width()),
            selectedTile: null,
            modal: null,
        },
        future: [],
    };
}

export class StateManager {
    private nile: WasmNile;
    private fullState: IState;
    private dispatch: React.Dispatch<Action>;

    public constructor(nile: WasmNile, fullState: IState, dispatch: React.Dispatch<Action>) {
        this.nile = nile;
        this.fullState = fullState;
        this.dispatch = dispatch;
    }

    public static useStateManager(nile: WasmNile): [IInnerState, StateManager] {
        // this isn't a react component
        // eslint-disable-next-line react-hooks/rules-of-hooks
        const [fullState, dispatch] = React.useReducer(StateManager.reducer, nile, initState);
        const sm = new StateManager(nile, fullState, dispatch);
        return [sm.state, sm];
    }

    public get state(): IInnerState {
        return this.fullState.now
    }

    public get canUndo(): boolean {
        return this.fullState.past.length > 0;
    }

    public get canRedo(): boolean {
        return this.fullState.future.length > 0;
    }

    public get selectedIsUniversal(): boolean {
        if (this.state.selectedTile?.type === "board") {
            const [row, column] = this.state.selectedTile.coordinates;
            return this.state.board[row][column].tilePlacement?.isUniversal ?? false;
        }
        return false;
    }

    public selectRackTile(idx: number): void {
        const rack = this.state.playerData[this.state.currentPlayerId].tileRack;
        if (idx < rack.length) {
            const tile = rack[idx];
            const isUniversal = tile === Tile.Universal;
            const tilePath = isUniversal ? TilePath.Straight : TilePathType.tile_into_normal(tile).tile_path();
            this.dispatch({type: "selectRackTile", idx, isUniversal, tilePath});
        }
    }

    public selectBoardTile(coordinates: CoordinateTuple): void {
        this.dispatch({type: "selectBoardTile", coordinates});
    }

    public placeOnBoard(row: number, column: number): void {
        if (this.state.selectedTile) {
            switch (this.state.selectedTile.type) {
                case "rack": {
                    try {
                        const rotation = Rotation.None;
                        const tile = {...this.state.selectedTile.tile};
                        const tilePathType = tile.isUniversal
                            ? TilePathType.universal(tile.tilePath)
                            : TilePathType.normal(tile.tilePath);
                        const score = this.nile.place_tile(tilePathType, new Coordinates(row, column), rotation);
                        this.dispatch({
                            type: "placeTile",
                            draggedTile: tile, coordinates: [row, column],
                            rotation, score,
                        });
                    } catch (err) {
                        this.dispatch({type: "setError", err});
                    }
                    return;
                }
                case "board": {
                    const [prevRow, prevColumn] = this.state.selectedTile.coordinates;
                    const oldCell = this.state.board[prevRow][prevColumn];
                    if (oldCell.tilePlacement) {
                        try {
                            const tilePlacement = oldCell.tilePlacement;
                            const score = this.nile.move_tile(
                                new Coordinates(prevRow, prevColumn),
                                new Coordinates(row, column)
                            );
                            this.dispatch({
                                type: "moveTile",
                                oldCoordinates: [prevRow, prevColumn],
                                newCoordinates: [row, column],
                                tilePlacement,
                                score,
                            });
                        } catch (err) {
                            this.dispatch({type: "setError", err});
                        }
                    } else {
                        this.dispatch({type: "setError", err: "Tried to move tile from cell with no tile"});
                    }
                    return;
                }
            }
        }
    }

    public rotate(isClockwise: boolean): void {
        if(this.state.selectedTile?.type === "board") {
            const [row, column] = this.state.selectedTile.coordinates;
            const cell = this.state.board[row][column];
            if(cell.tilePlacement) {
                try {
                    const newRotation = mod(cell.tilePlacement.rotation + (isClockwise ? 1 : -1), 4)    // 4 different rotations
                    this.nile.rotate_tile(new Coordinates(row, column), newRotation);
                    this.dispatch({type: "rotateTile", coordinates: [row, column], rotation: newRotation});
                } catch (err) {
                    this.dispatch({type: "setError", err});
                }
            }
        }
    }

    public removeSelectedTile(): void {
        if(this.state.selectedTile?.type === "board") {
            const [row, column] = this.state.selectedTile.coordinates;
            const cell = this.state.board[row][column];
            if(cell.tilePlacement) {
                try {
                    const score = this.nile.remove_tile(new Coordinates(row, column));
                    this.dispatch({type: "removeTile", coordinates: [row, column], score});
                } catch (err) {
                    this.dispatch({type: "setError", err});
                }
            }
        }
    }

    public updateUniversalPath(tilePath: TilePath): void {
        if(this.state.selectedTile?.type === "board") {
            const [row, column] = this.state.selectedTile.coordinates;
            const cell = this.state.board[row][column];
            if(cell.tilePlacement && cell.tilePlacement.isUniversal) {
                try {
                    this.nile.update_universal_path(new Coordinates(row, column), tilePath);
                    const tilePlacement = {...cell.tilePlacement, tilePath};
                    this.dispatch({type: "updateUniversalPath", coordinates: [row, column], tilePlacement});
                } catch (err) {
                    this.dispatch({type: "setError", err});
                }
            }
        }
    }

    public endTurn(): void {
        try {
            const endTurnUpdate = this.nile.end_turn();
            this.dispatch({type: "endTurn", endTurnUpdate});
        } catch(err) {
            this.dispatch({type: "setError", err});
        }
    }

    public cantPlay(): void {
        try {
            const endTurnUpdate = this.nile.cant_play();
            /// Save event as endTurn
            this.dispatch({type: "endTurn", endTurnUpdate});
        } catch(err) {
            this.dispatch({type: "setError", err});
        }
    }

    public undo(): void {
        try {
            this.nile.undo();
            this.dispatch({type: "undo"});
        } catch (e) {
            console.warn(`Error: ${e.message};`);
        }
    }

    public redo(): void {
        try {
            this.nile.redo();
            this.dispatch({type: "redo"});
        } catch (e) {
            console.warn(`Error: ${e.message};`);
        }
    }

    public takeCpuTurn(): void {
        if (this.state.playerData[this.state.currentPlayerId].isCpu) {
            const cpuUpdate = this.nile.take_cpu_turn();
            if (cpuUpdate) {
                this.dispatch({type: "cpuTurn", cpuUpdate});
            }
        }
    }

    public dismiss(): void {
        this.dispatch({type: "dismiss"});
    }

    private static reducer(prevState: IState, action: Action): IState {
        const state = prevState.now;
        switch (action.type) {
            case "selectRackTile":
                return update(prevState, {...state, selectedTile: {type: "rack", tile: action}});
            case "selectBoardTile":
                return update(prevState, {...state, selectedTile: {type: "board", coordinates: action.coordinates}});
            case "placeTile":
                return StateManager.reducePlaceTile(
                    prevState, action.draggedTile, action.coordinates, action.rotation, action.score
                );
            case "rotateTile":
                return StateManager.reduceRotateTile(prevState, action.coordinates, action.rotation);
            case "removeTile":
                return StateManager.reduceRemoveTile(prevState, action.coordinates, action.score);
            case "updateUniversalPath": {
                const board = updateCell(state.board, action.coordinates, (cell) => {
                    cell.tilePlacement = action.tilePlacement;
                    return cell;
                });
                return undoableUpdate(prevState, {...state, board});
            }
            case "moveTile":
                return StateManager.reduceMoveTile(
                    prevState, action.oldCoordinates, action.newCoordinates, action.tilePlacement,
                    action.score,
                );
            case "endTurn":
                return StateManager.reduceEndTurn(prevState, action.endTurnUpdate)
            case "undo": {
                if (prevState.past.length > 0) {
                    const [now, ...past] = prevState.past;
                    return {
                        past,
                        now,
                        future: [prevState.now, ...prevState.future],
                    };
                }
                return prevState;
            }
            case "redo":
                if (prevState.future.length > 0) {
                    const [now, ...future] = prevState.future;
                    return {
                        past: [prevState.now, ...prevState.past],
                        now,
                        future,
                    };
                }
                return prevState;
            case "cpuTurn":
                return StateManager.reduceCpuTurn(prevState, action.cpuUpdate);
            case "setError":
                return StateManager.reduceSetError(prevState, action.err);
            case "setEndOfGame":
                return update(prevState, {...state, modal: {type: "endOfGame", msg: action.msg}});
            case "dismiss":
                return update(prevState, {...state, modal: null});
            default:
                return prevState;
        }
    }

    private static reducePlaceTile(
        prevState: IState,
        tile: IRackDraggedTile,
        coordinates: CoordinateTuple,
        rotation: Rotation,
        score: TurnScore,
    ): IState {
        const state = prevState.now;
        // Place tile on board
        const board = updateCell(state.board, coordinates, (cell) => {
            cell.tilePlacement = {
                ...tile,
                rotation,
            };
            return cell;
        });

        const playerData = updatePlayer(state.playerData, state.currentPlayerId, (player) => {
            // Update scores
            player.currentTurnScore = score;
            // Remove tile from tile rack
            const tileRack = [...player.tileRack];
            tileRack.splice(tile.idx, 1);
            player.tileRack = tileRack;
            return player;
        });
        // Add to currentTurnTiles
        const currentTurnTiles = state.currentTurnTiles.add(coordinates);
        // Update selectedTile
        const selectedTile: SelectedTile = {type: "board", coordinates: coordinates};
        return undoableUpdate(
            prevState,
            {...state, board, playerData, currentTurnTiles, selectedTile}
        );
    }

    private static reduceRotateTile(
        prevState: IState,
        coordinates: CoordinateTuple,
        rotation: Rotation,
    ): IState {
        const state = prevState.now;
        let board;
        try {
            board = updateCell(state.board, coordinates, (cell) => {
                if(cell.tilePlacement === null) {
                    console.warn("Tried to rotate empty tile");
                    throw new Error("Tried to rotate empty tile");
                }
                cell.tilePlacement = {...cell.tilePlacement, rotation};
                return cell;
            });
        } catch {
            return prevState;
        }

        return undoableUpdate(prevState, {...state, board});
    }

    private static reduceRemoveTile(
        prevState: IState,
        coordinates: CoordinateTuple,
        score: TurnScore,
    ): IState {
        const state = prevState.now;
        // Remove tile from board
        const [i, j] = coordinates;
        const tilePlacement = state.board[i][j].tilePlacement;
        if (tilePlacement !== null) {
            const board = updateCell(state.board, coordinates, (cell) => {
                cell.tilePlacement = null;
                return cell;
            });

            const playerData = updatePlayer(state.playerData, state.currentPlayerId, (player) => {
                // Update scores
                player.currentTurnScore = score;
                // Return tile from tile rack
                player.tileRack = [
                    ...player.tileRack,
                    tilePlacement.isUniversal
                    ? Tile.Universal
                    : tile_path_to_tile(tilePlacement.tilePath)
                ];
                return player;
            });
            // Remove from currentTurnTiles
            const currentTurnTiles = state.currentTurnTiles.delete(coordinates);
            // Update selectedTile
            const selectedTile = null;
            return undoableUpdate(
                prevState, {
                    ...state, board, playerData, currentTurnTiles, selectedTile
                }
            );
        }
        return prevState;
    }

    private static reduceMoveTile(
        prevState: IState,
        oldCoordinates: CoordinateTuple,
        newCoordinates: CoordinateTuple,
        tilePlacement: TilePlacement,
        score: TurnScore
    ): IState {
        const state = prevState.now;
        let board = updateCell(state.board, oldCoordinates, (cell) => {
            cell.tilePlacement = null;
            return cell;
        });
        board = updateCell(board, newCoordinates, (cell) => {
            cell.tilePlacement = tilePlacement;
            return cell;
        });
        const playerData = updatePlayer(state.playerData, state.currentPlayerId, (player) => {
            player.currentTurnScore = score;
            return player;
        })
        const currentTurnTiles = state.currentTurnTiles
            .delete(oldCoordinates)
            .add(newCoordinates);

        return undoableUpdate(prevState, {
            ...state,
            board,
            playerData,
            selectedTile: {type: "board", coordinates: newCoordinates},
            currentTurnTiles,
        });
    }

    private static reduceEndTurn(prevState: IState, update: EndTurnUpdate): IState {
        const state = prevState.now;
        const playerData = updatePlayer(state.playerData, state.currentPlayerId, (player) => {
            // Update scores
            player.scores = [...player.scores, update.turn_score];
            player.currentTurnScore = {add: 0, sub: 0};
            player.tileRack = update.get_tiles();
            return player;
        });
        const currentPlayerId = mod(state.currentPlayerId + 1, state.playerData.length);
        const modal: Modal | null = update.game_has_ended
            ? {type: "endOfGame", msg: `${maxBy(state.playerData, (p) => sumTurnScores(p.scores))?.name} has won`}
            : state.modal;
        return updateAndReset({
            ...state,
            playerData,
            currentPlayerId,
            gameHasEnded: update.game_has_ended,
            selectedTile: null,
            currentTurnTiles: state.currentTurnTiles.clear(),
            modal,
        });
    }

    private static reduceCpuTurn(prevState: IState, cpuUpdate: CPUTurnUpdate): IState {
        const state = prevState.now;
        const playerData = updatePlayer(state.playerData, cpuUpdate.player_id, (player) => {
            player.scores = [...player.scores, cpuUpdate.turn_score];
            player.tileRack = new Array(cpuUpdate.tile_count).fill(Tile.Straight);
            return player;
        });
        const currentPlayerId = mod(state.currentPlayerId + 1, state.playerData.length);
        let board = state.board;
        cpuUpdate.get_placements().forEach((placement: TilePlacementEvent) => {
            const coordinates = placement.get_coordinates();
            const tilePathType = placement.get_tile_path_type();
            board = updateCell(board, [coordinates[0], coordinates[1]], (cell) => {
                cell.tilePlacement = {
                    isUniversal: tilePathType.is_universal(),
                    rotation: placement.get_rotation(),
                    tilePath: tilePathType.tile_path()
                };
                return cell;
            });
        });
        const modal: Modal | null = cpuUpdate.game_has_ended
            ? {type: "endOfGame", msg: `${maxBy(state.playerData, (p) => sumTurnScores(p.scores))?.name} has won`}
            : state.modal;
        return updateAndReset({
            ...state,
            playerData,
            currentPlayerId,
            board,
            gameHasEnded: cpuUpdate.game_has_ended,
            modal,
        });
    }

    private static reduceSetError(prevState: IState, err: unknown): IState {
        const state = prevState.now;
        if (typeof err === "string") {
            return update(prevState, {...state, modal: {type: "error", msg: err}});
        }
        if (err instanceof Error) {
            return update(prevState, {...state, modal: {type: "error", msg: err.message}});
        }
        return update(prevState, {...state, modal: {type: "error", msg: `${err}`}});
    }
}

const update = (prevState: IState, newState: IInnerState): IState => ({
    ...prevState,
    now: newState,
});
const undoableUpdate = (prevState: IState, newState: IInnerState): IState => ({
    past: [prevState.now, ...prevState.past],
    now: newState,
    future: [],
});
const updateAndReset = (newState: IInnerState): IState => ({
    past: [],
    now: newState,
    future: [],
});
const updateCell = (
    prevBoard: BoardArray,
    coordinates: CoordinateTuple,
    cellReducer: (cell: Cell) => Cell
): BoardArray => {

    const [i, j] = coordinates;
    const board = [...prevBoard];
    const column = [...board[i]];
    const cell = cellReducer({...column[j]});
    column[j] = cell;
    board[i] = column;
    return board;
}
const updatePlayer = (
    playerData: PlayerData[],
    currentPlayerId: number,
    playerReducer: (player: PlayerData) => PlayerData
): PlayerData[] => {

    const playerDataArray = [...playerData];
    const player = playerReducer({...playerDataArray[currentPlayerId]});
    playerDataArray[currentPlayerId] = player;
    return playerDataArray;
}
