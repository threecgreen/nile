import { Rotation, Tile, TilePath, tile_path_to_tile, TurnScore, WasmNile } from "nile";
import { BoardArray, Cell, CoordinateTuple, PlayerData, TilePlacement, toBoardArray, toPlayerDataArray } from "./common";
import { mod } from "./utils";

interface IRackDraggedTile {
    idx: number;
    tilePath: TilePath;
    isUniversal: boolean;
}

interface IInnerState {
    nile: WasmNile;
    board: BoardArray;
    currentPlayerId: number;
    playerData: PlayerData[];
    /** A tile in the process of being dragged */
    draggedTile: IRackDraggedTile | null;
    /** Used for determining if placed tile is movable, rotatable, etc. */
    currentTurnTiles: CoordinateTuple[];
    /** Coordinates the selected tile on the board */
    selectedTile: CoordinateTuple | null;
}

interface IState {
    past: IInnerState[];
    now: IInnerState;
    future: IInnerState[];
}

type Action =
    | {type: "setDraggedTile", tilePath: TilePath, isUniversal: boolean, idx: number}
    | {type: "selectTile", coordinates: CoordinateTuple}
    | {type: "placeTile", draggedTile: IRackDraggedTile, coordinates: CoordinateTuple, rotation: Rotation, score: TurnScore}
    | {type: "rotateTile", coordinates: CoordinateTuple, rotation: Rotation}
    | {type: "removeTile", coordinates: CoordinateTuple, score: TurnScore}
    | {type: "updateUniversalPath", coordinates: CoordinateTuple, tilePlacement: TilePlacement}
    | {type: "moveTile", oldCoordinates: CoordinateTuple, newCoordinates: CoordinateTuple, tilePlacement: TilePlacement, score: TurnScore}
    | {type: "undo"}
    | {type: "redo"}
    | {type: "endTurn", turnScore: TurnScore, tiles: Tile[]}

export const initState = (playerNames: string[], aiPlayerCount: number): IState => {
    // TODO: move WasmNile behind interface for easier testing
    const nile = new WasmNile(playerNames, aiPlayerCount);
    return {
        past: [],
        now: {
            nile,
            board: toBoardArray(nile.board()),
            currentPlayerId: 0,
            playerData: toPlayerDataArray(nile.players()),
            draggedTile: null,
            currentTurnTiles: [],
            selectedTile: null,
        },
        future: [],
    };
}

export const reducer: React.Reducer<IState, Action> = (prevState, action) => {
    const state = prevState.now;
    switch (action.type) {
        case "setDraggedTile":
            return update(prevState, {...state, draggedTile: {...action}});
        case "selectTile":
            return update(prevState, {...state, selectedTile: action.coordinates});
        case "placeTile": {
            // Place tile on board
            const board = updateCell(state.board, action.coordinates, (cell) => {
                cell.tilePlacement = {
                    tilePath: action.draggedTile.tilePath,
                    isUniversal: action.draggedTile.isUniversal,
                    rotation: action.rotation,
                };
                return cell;
            });

            const playerData = updatePlayer(state.playerData, state.currentPlayerId, (player) => {
                // Update scores
                player.currentTurnScore = {add: action.score.add(), sub: action.score.sub()};
                // Remove tile from tile rack
                const tileRack = [...player.tileRack];
                tileRack.splice(action.draggedTile.idx, 1);
                player.tileRack = tileRack;
                return player;
            });
            // Add to currentTurnTiles
            const currentTurnTiles = [...state.currentTurnTiles, action.coordinates];
            // Update selectedTile
            const selectedTile = action.coordinates;
            return undoableUpdate(
                prevState,
                {...state, board, playerData, currentTurnTiles, selectedTile, draggedTile: null}
            );
        }
        case "rotateTile": {
            let board;
            try {
                board = updateCell(state.board, action.coordinates, (cell) => {
                    if(cell.tilePlacement === null) {
                        console.warn("Tried to rotate empty tile");
                        throw new Error("Tried to rotate empty tile");
                    }
                    cell.tilePlacement = {...cell.tilePlacement, rotation: action.rotation};
                    return cell;
                });
            } catch {
                return prevState;
            }

            return undoableUpdate(prevState, {...state, board});
        }
        case "removeTile": {
            // Remove tile from board
            const [i, j] = action.coordinates;
            const tilePlacement = state.board[i][j].tilePlacement;
            if (tilePlacement !== null) {
                const tilePath = tilePlacement.tilePath;
                const board = updateCell(state.board, action.coordinates, (cell) => {
                    cell.tilePlacement = null;
                    return cell;
                });

                const playerData = updatePlayer(state.playerData, state.currentPlayerId, (player) => {
                    // Update scores
                    player.currentTurnScore = {add: action.score.add(), sub: action.score.sub()};
                    // Return tile from tile rack
                    player.tileRack = [...player.tileRack, tile_path_to_tile(tilePath)];
                    return player;
                });
                // Remove from currentTurnTiles
                const currentTurnTiles = state.currentTurnTiles.filter(([ci, cj]) => ci !== i && cj !== j);
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
        case "updateUniversalPath": {
            const board = updateCell(state.board, action.coordinates, (cell) => {
                cell.tilePlacement = action.tilePlacement;
                return cell;
            });
            return undoableUpdate(prevState, {...state, board});
        }
        case "moveTile": {
            let board = updateCell(state.board, action.oldCoordinates, (cell) => {
                cell.tilePlacement = null;
                return cell;
            });
            board = updateCell(board, action.newCoordinates, (cell) => {
                cell.tilePlacement = action.tilePlacement;
                return cell;
            });
            const playerData = updatePlayer(state.playerData, state.currentPlayerId, (player) => {
                player.currentTurnScore = {add: action.score.add(), sub: action.score.sub()};
                return player;
            })
            const currentTurnTiles = state.currentTurnTiles.filter(
                ([ci, cj]) => ci !== action.oldCoordinates[0] && cj !== action.oldCoordinates[1]);
            currentTurnTiles.push(action.newCoordinates);

            return undoableUpdate(prevState, {...state, board, playerData, selectedTile: action.newCoordinates, currentTurnTiles});
        }
        case "endTurn": {
            const playerData = updatePlayer(state.playerData, state.currentPlayerId, (player) => {
                // Update scores
                player.scores = [...player.scores, {add: action.turnScore.add(), sub: action.turnScore.sub()}];
                player.currentTurnScore = {add: 0, sub: 0};
                player.tileRack = action.tiles.map((t) =>
                    // @ts-ignore
                    Tile[t] as Tile
                );
                return player;
            });
            const currentPlayerId = mod(state.currentPlayerId + 1, state.playerData.length);
            return updateAndReset({
                ...state,
                // // FIXME: temporary to test AI
                // board: toBoardArray(state.nile.board()),
                // playerData: toPlayerDataArray(state.nile.players()),

                playerData,
                currentPlayerId,
                draggedTile: null,
                selectedTile: null,
                currentTurnTiles: [],
            });
        }
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
        default:
            return prevState;
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