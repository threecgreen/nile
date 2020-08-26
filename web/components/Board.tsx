import { BoardArray, CoordinateTuple } from "lib/common";
import React from "react";
import styles from "./Board.module.css";
import { GridCell } from "./Grid";
import { EmptyTile, Tile } from "./Tile";

interface IProps {
    board: BoardArray;
    selectedTile: CoordinateTuple | null;
    currentTurnTiles: CoordinateTuple[];
    onDropFromRack: (row: number, column: number) => void;
    onSelect: (coordinates: CoordinateTuple) => void;
    onDragStart: (coordinates: CoordinateTuple) => void;
}

export const Board: React.FC<IProps> = ({board, selectedTile, currentTurnTiles, onDropFromRack, onSelect, onDragStart}) => {
    const onDrag = (e: React.DragEvent) => {
        e.preventDefault();
    }

    return (
        <table className={ styles.board }>
            <tbody>
                { board.map((row, i) => (
                    <tr key={ i }>
                        <td>
                            {/* Start arrow  */}
                            { i == 10 ? <p className={ styles.arrow}>→</p> : null }
                        </td>
                        { row.map((cell, j) => (
                            <GridCell key={ j }>
                                { cell.tilePlacement
                                ? <div draggable={ currentTurnTiles.some(([ci, cj]) => ci === i && cj === j) }
                                    onDrag={ onDrag }
                                    onDragStart={ (_) => onDragStart([i, j]) }
                                >
                                    <Tile tilePath={ cell.tilePlacement.tilePath }
                                        isUniversal={ cell.tilePlacement.isUniversal }
                                        onSelect={ () => onSelect([i, j]) }
                                        isSelected={ selectedTile !== null
                                            && i === selectedTile[0] && j === selectedTile[1] }
                                        rotation={ cell.tilePlacement.rotation }
                                    />
                                </div>
                                : <EmptyTile bonus={ cell.bonus }
                                    isEndGame={ j === (board[0].length - 1) }
                                    onDrop={ () => onDropFromRack(i, j) }
                                /> }
                            </GridCell>
                        )) }
                    </tr>
                )) }
            </tbody>
        </table>
    );
};
Board.displayName = "Board";
