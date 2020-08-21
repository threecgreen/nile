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
    <table className={ styles.board }>
        <tbody>
            { board.map((row, i) => (
                <tr key={ i }>
                    <td>
                        { i == 10 ? <p className={ styles.arrow}>→</p> : null }
                    </td>
                    { row.map((cell, j) => (
                        <GridCell key={ j }>
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
                    )) }
                </tr>
            )) }
        </tbody>
    </table>
);
Board.displayName = "Board";
