import { Board, Player, Rotation, Tile, TilePath, TilePathType, tile_score } from "nile";
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
        const isCpu = p.is_cpu();
        return {
            name: p.get_name(),
            tileRack: isCpu
                ? new Array(p.get_tiles().length).fill(Tile.Straight)
                : p.get_tiles().map((t) => (
                    t as Tile
                )),
            scores: p.total_score() !== 0 ? [{add: p.total_score(), sub: 0}] : [],
            currentTurnScore: {add: 0, sub: 0},
            isCpu,
        };
    })
)

/** Memoized cache of tile scores */
export const score = (() => {
    const cache = new Map();
    return (tile: Tile) => {
        if (cache.has(tile)) {
            return cache.get(tile);
        }
        const res = tile_score(tile);
        cache.set(tile, res);
        return res;
    };
})();
