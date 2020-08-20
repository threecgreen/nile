import { Board, Tile, Rotation, Player } from "nile";
import { range } from "./utils";

export type CoordinateTuple = [number, number];
export type TilePlacement = {
    tile: Tile;
    rotation: Rotation;
}
export type Cell = {
    bonus: number;
    tilePlacement: TilePlacement | null;
}
export type BoardArray = Array<Array<Cell>>;

export const toBoardArray = (board: Board): BoardArray => {
    const boardArray: BoardArray = [];
    for (const i of range({stop: board.height()})) {
        boardArray.push([]);
        for (const j of range({stop: board.width()})) {
            const cell = board.get_cell(i, j);

            const optTile = cell.tile();
            if (optTile) {
                boardArray[i].push({
                    bonus: cell.bonus(),
                    tilePlacement: {
                        // @ts-ignore
                        tile: Tile[optTile.tile],
                        // @ts-ignore
                        rotation: Rotation[optTile.rotation],
                    },
                });
            } else {
                boardArray[i].push({
                    bonus: cell.bonus(),
                    tilePlacement: null,
                });
            }
        }
    }
    return boardArray;
}

export type TurnScore = {
    add: number;
    sub: number;
}

export type PlayerData = {
    name: string;
    tileRack: Tile[];
    scores: TurnScore[];
    currentTurnScore: TurnScore;
}

export const toPlayerDataArray = (players: Player[]): PlayerData[] => (
    players.map((p) => ({
        name: p.get_name(),
        tileRack: p.get_tiles().map((t) => (
            // @ts-ignore
            Tile[t] as number
        )),
        scores: [],
        currentTurnScore: {add: 0, sub: 0},
    }))
)
