import { BoardArray } from "lib/common";
import React from "react";
import styles from "./Board.module.css";
import { GridCell } from "./Grid";
import { EmptyTile, Tile } from "./Tile";

interface IProps {
    board: BoardArray;
    onDropFromRack: (row: number, column: number) => void;
}

export const Board: React.FC<IProps> = ({board, onDropFromRack: onDrop}) => (
    <div className={ styles.board }>
        { board.map((row, i) => (
            row.map((cell, j) => (
                <GridCell key={ `${i} | ${j}` }
                    row={ i }
                    column={ j }
                >
                    { cell.tilePlacement
                    ? <Tile tile={ cell.tilePlacement.tile }
                        rotation={ cell.tilePlacement.rotation }
                    />
                    : <EmptyTile onDrop={ () => onDrop(i, j) } /> }
                </GridCell>
            ))
        )) }
    </div>
);
Board.displayName = "Board";
