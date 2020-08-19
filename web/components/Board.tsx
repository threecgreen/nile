import { range } from "lib/utils";
import { Board as BoardState } from "nile";
import React from "react";
import { EmptyTile, Tile } from "./Tile";
import styles from "./Board.module.css";
import { GridCell } from "./Grid";

interface IProps {
    board: BoardState;
    height: number;
    width: number;
    onDropFromRack: (row: number, column: number) => void;
}

export const Board: React.FC<IProps> = ({board, height, width, onDropFromRack: onDrop}) => (
    <div className={ styles.board }>
        { Array.from(range({stop: height})).map((row) => (
            Array.from(range({stop: width})).map((col) => {
                const cell = board.get_cell(row, col);
                const tile = cell.tile();
                return (
                    <GridCell key={ `${row} | ${col}` }
                        row={ row }
                        column={ col }
                    >
                        { tile
                        ? <Tile tile={ tile.tile }
                            rotation={ tile.rotation }
                        />
                        : <EmptyTile onDrop={ () => onDrop(row, col) } /> }
                    </GridCell>
                );
            })
        )) }
    </div>
);
Board.displayName = "Board";
