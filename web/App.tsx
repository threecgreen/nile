import React from "react";
import { Tile as TileEnum, Rotation, WasmNile } from "nile";
import { Tile, EmptyTile } from "components/Tile";
import styles from "./App.module.css";

interface IRangeArgs {
    start?: number;
    stop?: number;
    step?: number;
}
/**
 * Creates an iterable range of numbersonClick.
 * @param start First number of the range
 * @param stop End of the range (exclusive)
 * @param step Increment of the range
 */
export function* range({ start, stop, step }: IRangeArgs): IterableIterator<number> {
    step = step || 1;
    start = start || 0;
    stop = stop || Number.MAX_SAFE_INTEGER;
    for (let i = start; i < stop; i += step) {
        yield i;
    }
}

export const App: React.FC = () => {
    const [nile, _] = React.useState(() => new WasmNile(["player1", "player2"]));
    const board = nile.board();
    const width = board.width();
    const height = board.height();
    return (
        <div className={ styles.board }>
            { Array.from(range({stop: height})).map((row) => (
                Array.from(range({stop: width})).map((col) => {
                    const cell = board.get_cell(row, col);
                    const tile = cell.tile();
                    return tile
                        ? <Tile row={ row }
                            key={ `${row} | ${col}` }
                            column={ col }
                            totalColumns={ width }
                            tile={ tile.tile }
                            rotation={ tile.rotation }
                        />
                        : <EmptyTile row={ row }
                            key={ `${row} | ${col}` }
                            column={ col }
                            totalColumns={ width }
                        />
                })
            )) }
        </div>
    );
};
App.displayName = "App";
