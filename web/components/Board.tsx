import { BoardArray, CoordinateTuple } from "lib/common";
import React from "react";
import styles from "./Board.module.css";
import { GridCell } from "./Grid";
import { EmptyTile, Tile, TileType } from "./Tile";

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
        <div className={ styles.outer }>
            {/* Start arrow  */}
            <span className={ styles.start }>Start</span>
            <span className={ styles.arrow }>→</span>
            <table className={ styles.board }>
                <tbody>
                    { board.map((row, i) => (
                        <tr key={ i }>
                            { row.map((cell, j) => {
                                const isEndGame = j === (board[0].length - 1);
                                const type = isEndGame
                                    ? TileType.EndGame
                                    : cell.bonus > 0 ? TileType.Bonus
                                    : cell.bonus < 0 ? TileType.Penalty
                                    : TileType.Normal;
                                return (
                                    <GridCell key={ j }>
                                        { cell.tilePlacement
                                        ? <div draggable={ currentTurnTiles.some(([ci, cj]) => ci === i && cj === j) }
                                            onDrag={ onDrag }
                                            onDragStart={ (_) => onDragStart([i, j]) }
                                        >
                                            <Tile tilePath={ cell.tilePlacement.tilePath }
                                                isUniversal={ cell.tilePlacement.isUniversal }
                                                isSelected={ selectedTile !== null
                                                    && i === selectedTile[0] && j === selectedTile[1] }
                                                rotation={ cell.tilePlacement.rotation }
                                                type={ type }
                                                onSelect={ () => onSelect([i, j]) }
                                            />
                                        </div>
                                        : <EmptyTile bonus={ cell.bonus }
                                            isEndGame={ isEndGame }
                                            onDrop={ () => onDropFromRack(i, j) }
                                        /> }
                                    </GridCell>
                                );
                            }) }
                        </tr>
                    )) }
                </tbody>
            </table>
        </div>
    );
};
Board.displayName = "Board";
