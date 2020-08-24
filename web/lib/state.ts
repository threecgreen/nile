import { Rotation, Tile, TurnScore, WasmNile, TilePath } from "nile";
import { BoardArray, Cell, CoordinateTuple, PlayerData, toBoardArray, toPlayerDataArray, TilePlacement } from "./common";
import { mod } from "./utils";

interface IDraggedTile {
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
    draggedTile: IDraggedTile | null;
    /** Used for determining if placed tile is movable, rotatable, etc. */
    currentTurnTiles: Array<CoordinateTuple>;
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
    | {type: "updateUniversalPath", coordinates: CoordinateTuple, tilePlacement: TilePlacement}
    | {type: "placeTile", draggedTile: IDraggedTile, coordinates: CoordinateTuple, rotation: Rotation, score: TurnScore}
    | {type: "rotateTile", coordinates: CoordinateTuple, rotation: Rotation}
    | {type: "removeTile"}
    | {type: "undo"}
    | {type: "redo"}
    | {type: "endTurn", turnScore: TurnScore, tiles: Tile[]}

export const initState = (playerNames: string[]): IState => {
    const nile = new WasmNile(playerNames);
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

export const reducer: React.Reducer<IState, Action> = (prevState, action) => {
    const state = prevState.now;
    switch (action.type) {
        case "setDraggedTile":
            return update(prevState, {...state, draggedTile: {...action}});
        case "selectTile":
            return update(prevState, {...state, selectedTile: action.coordinates});
        case "updateUniversalPath": {
            const [i, j] = action.coordinates;
            const board = [...state.board];
            const column = [...board[i]];
            const cell: Cell = {...column[j], tilePlacement: action.tilePlacement};
            column[j] = cell;
            board[i] = column;
            return undoableUpdate(prevState, {...state, board});
        }
        case "placeTile": {
            // Place tile on board
            const [i, j] = action.coordinates;
            const board = [...state.board];
            const column = [...board[i]];
            const cell: Cell = {...column[j], tilePlacement: {
                tilePath: action.draggedTile.tilePath,
                isUniversal: action.draggedTile.isUniversal,
                rotation: action.rotation,
            }};
            column[j] = cell;
            board[i] = column;

            const playerDataArray = [...state.playerData];
            const playerData: PlayerData = {...playerDataArray[state.currentPlayerId]};
            // Update scores
            playerData.currentTurnScore = {add: action.score.add(), sub: action.score.sub()};
            // Remove tile from tile rack
            const tileRack = [...playerData.tileRack];
            tileRack.splice(action.draggedTile.idx, 1);
            playerData.tileRack = tileRack;
            playerDataArray[state.currentPlayerId] = playerData;
            // Add to currentTurnTiles
            const currentTurnTiles = [...state.currentTurnTiles, action.coordinates];
            // Update selectedTile
            const selectedTile = action.coordinates;
            return undoableUpdate(
                prevState,
                {...state, board, playerData: playerDataArray, currentTurnTiles, selectedTile}
            );
        }
        case "rotateTile": {
            const [i, j] = action.coordinates;
            const board = [...state.board];
            const column = [...board[i]];
            if(column[j].tilePlacement === null) {
                console.warn("Tried to rotate empty tile");
                return prevState;
            }
            const cell: Cell = {...column[j], tilePlacement: {
                ...column[j].tilePlacement!,
                rotation: action.rotation,
            }};
            column[j] = cell;
            board[i] = column;

            return undoableUpdate(prevState, {...state, board});
        }
        case "endTurn": {
            const playerDataArray = [...state.playerData];
            const playerData: PlayerData = {...playerDataArray[state.currentPlayerId]};
            // Update scores
            playerData.currentTurnScore = {add: action.turnScore.add(), sub: action.turnScore.sub()};
            playerData.tileRack = action.tiles.map((t) =>
                // @ts-ignore
                Tile[t] as Tile
            );
            playerDataArray[state.currentPlayerId] = playerData;
            const currentPlayerId = mod(state.currentPlayerId + 1, state.playerData.length);
            return updateAndReset({
                ...state,
                playerData: playerDataArray,
                currentPlayerId,
                draggedTile: null,
                selectedTile: null,
                currentTurnTiles: []
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
            // fallthrough
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
            // fallthrough
        default:
            return prevState;
    }
}
