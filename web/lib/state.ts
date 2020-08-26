import { Rotation, Tile, TilePath, tile_path_to_tile, TurnScore, WasmNile, CPUTurnUpdate, TilePlacementEvent } from "nile";
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
    /** Whether the game has finished */
    gameHasEnded: boolean;
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
    /** Same event for cantPlay */
    | {type: "endTurn", turnScore: TurnScore, tiles: Tile[], hasEnded: boolean}
    | {type: "cpuTurn", cpuUpdate: CPUTurnUpdate}

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
            gameHasEnded: false,
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
                player.currentTurnScore = action.score;
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
                    player.currentTurnScore = action.score;
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
                player.currentTurnScore = action.score;
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
                player.scores = [...player.scores, action.turnScore];
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
                playerData,
                currentPlayerId,
                gameHasEnded: action.hasEnded,
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
        case "cpuTurn": {
            const playerData = updatePlayer(state.playerData, action.cpuUpdate.player_id, (player) => {
                player.scores = [...player.scores, action.cpuUpdate.turn_score];
                return player;
            });
            const currentPlayerId = mod(state.currentPlayerId + 1, state.playerData.length);
            let board = state.board;
            action.cpuUpdate.get_placements().forEach((placement: TilePlacementEvent) => {
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
            return updateAndReset({
                ...state,
                playerData,
                currentPlayerId,
                board,
                gameHasEnded: action.cpuUpdate.game_has_ended,
             });
        }
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
