import { BoardArray, CoordinateTuple } from "lib/common";
import React from "react";
import styles from "./Board.module.css";
import { GridCell } from "./Grid";
import { EmptyTile, Tile } from "./Tile";

interface IProps {
    board: BoardArray;
    selectedTile: CoordinateTuple | null;
    onDropFromRack: (row: number, column: number) => void;
    onSelect: (coordinates: CoordinateTuple) => void;
}

export const Board: React.FC<IProps> = ({board, selectedTile, onDropFromRack, onSelect}) => (
    <div className={ styles.board }>
        { board.map((row, i) => (
            row.map((cell, j) => (
                // TODO: maybe just use a table...
                <GridCell key={ `${i} | ${j}` }
                    row={ i }
                    column={ j }
                >
                    { cell.tilePlacement
                    ? <Tile tile={ cell.tilePlacement.tile }
                        onSelect={ () => onSelect([i, j]) }
                        isSelected={ selectedTile !== null
                            && i === selectedTile[0] && j === selectedTile[1] }
                        rotation={ cell.tilePlacement.rotation }
                    />
                    : <EmptyTile bonus={ cell.bonus }
                        onDrop={ () => onDropFromRack(i, j) }
                    /> }
                </GridCell>
            ))
        )) }
    </div>
);
Board.displayName = "Board";
