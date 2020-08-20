import { Rotation, Tile, TurnScore, WasmNile } from "nile";
import { BoardArray, Cell, CoordinateTuple, PlayerData, toBoardArray, toPlayerDataArray } from "./common";



interface IState {
    nile: WasmNile;
    board: BoardArray;
    currentPlayerId: number;
    playerData: PlayerData[];
    draggedTile: Tile | null;
    /** Used for determining if placed tile is movable, rotatable, etc. */
    currentTurnTiles: Array<[number, number]>;
}

type Action =
    | {type: "setDraggedTile", tile: Tile}
    | {type: "placeTile", tile: Tile, coordinates: CoordinateTuple, rotation: Rotation, score: TurnScore}
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
    };
}

export const reducer: React.Reducer<IState, Action> = (state, action) => {
    switch (action.type) {
        case "setDraggedTile":
            return {...state, draggedTile: action.tile};
        case "placeTile":
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
            playerData.currentTurnScore = {add: action.score.add(), sub: action.score.sub()};
            const tileRack = [...playerData.tileRack];
            const idx = playerData.tileRack.findIndex((t) => t === action.tile);
            tileRack.splice(idx, 1);
            playerData.tileRack = tileRack;
            playerDataArray[state.currentPlayerId] = playerData;

            console.log(state.board === board);
            console.log(state.board[i] === column);
            console.log(state.board[i][j] === cell);

            return {...state, board, playerData: playerDataArray};
        case "rotateTile":
        default:
            return state;
    }
}
