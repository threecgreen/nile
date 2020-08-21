import { Rotation, Tile, TurnScore, WasmNile } from "nile";
import { BoardArray, Cell, CoordinateTuple, PlayerData, toBoardArray, toPlayerDataArray } from "./common";

interface IDraggedTile {
    idx: number;
    tile: Tile;
}

interface IState {
    nile: WasmNile;
    board: BoardArray;
    currentPlayerId: number;
    playerData: PlayerData[];
    draggedTile: IDraggedTile | null;
    /** Used for determining if placed tile is movable, rotatable, etc. */
    currentTurnTiles: Array<CoordinateTuple>;
    selectedTile: CoordinateTuple | null;
}

type Action =
    | {type: "setDraggedTile", tile: Tile, idx: number}
    | {type: "selectTile", coordinates: CoordinateTuple}
    | {type: "placeTile", tile: Tile, coordinates: CoordinateTuple, rotation: Rotation, score: TurnScore, idx: number}
    | {type: "rotateTile", coordinates: CoordinateTuple, rotation: Rotation}
    | {type: "removeTile"}
    | {type: "undo"}
    | {type: "redo"}

export const initState = (playerNames: string[]): IState => {
    const nile = new WasmNile(playerNames);
    return {
        nile,
        board: toBoardArray(nile.board()),
        currentPlayerId: 0,
        playerData: toPlayerDataArray(nile.players()),
        draggedTile: null,
        currentTurnTiles: [],
        selectedTile: null,
    };
}

export const reducer: React.Reducer<IState, Action> = (state, action) => {
    switch (action.type) {
        case "setDraggedTile":
            return {...state, draggedTile: {idx: action.idx, tile: action.tile}};
        case "selectTile":
            return {...state, selectedTile: action.coordinates};
        case "placeTile": {
            // Place tile on board
            const [i, j] = action.coordinates;
            const board = [...state.board];
            const column = [...board[i]];
            const cell: Cell = {...column[j], tilePlacement: {
                tile: action.tile,
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
            tileRack.splice(action.idx, 1);
            playerData.tileRack = tileRack;
            playerDataArray[state.currentPlayerId] = playerData;
            // Add to currentTurnTiles
            const currentTurnTiles = [...state.currentTurnTiles, action.coordinates];
            // Update selectedTile
            const selectedTile = action.coordinates;
            return {...state, board, playerData: playerDataArray, currentTurnTiles, selectedTile};
        }
        case "rotateTile": {
            const [i, j] = action.coordinates;
            const board = [...state.board];
            const column = [...board[i]];
            if(column[j].tilePlacement === null) {
                console.warn("Tried to rotate empty tile");
                return state;
            }
            const cell: Cell = {...column[j], tilePlacement: {
                tile: column[j].tilePlacement!.tile,
                rotation: action.rotation,
            }};
            column[j] = cell;
            board[i] = column;

            return {...state, board};
        }
        default:
            return state;
    }
}
