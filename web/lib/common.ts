import { Board, Player, Rotation, Tile, TilePathType, TilePath } from "nile";
import { range } from "./utils";

export type CoordinateTuple = [number, number];
export type TilePlacement = {
    tilePath: TilePath;
    isUniversal: boolean;
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
        for (const j of range({stop: board.width() + 1})) {
            const cell = board.get_cell(i, j);

            const optTile = cell.tile();
            if (optTile) {
                const tilePathType: TilePathType = optTile.get_tile_path_type();
                boardArray[i].push({
                    bonus: cell.bonus(),
                    tilePlacement: {
                        tilePath: tilePathType.tile_path(),
                        isUniversal: tilePathType.is_universal(),
                        rotation: optTile.get_rotation(),
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

export const sumTurnScores = (scores: TurnScore[]): number => (
    scores.reduce((acc, ts) => acc + ts.add - ts.sub, 0)
);

export type PlayerData = {
    name: string;
    tileRack: Tile[];
    scores: TurnScore[];
    currentTurnScore: TurnScore;
    isCpu: boolean;
}

export const toPlayerDataArray = (players: Player[]): PlayerData[] => (
    players.map((p) => {
        return {
            name: p.get_name(),
            tileRack: p.get_tiles().map((t) => (
                // @ts-ignore
                Tile[t] as number
            )),
            scores: p.total_score() !== 0 ? [{add: p.total_score(), sub: 0}] : [],
            currentTurnScore: {add: 0, sub: 0},
            isCpu: p.is_cpu(),
        };
    })
)
