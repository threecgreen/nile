import { BoardArray, CoordinateTuple, Cell } from "lib/common";
import React from "react";
import styles from "./Board.module.css";
import { EmptyCell, TileCell, TileType } from "./Tile";
import { CoordinateSet } from "lib/CoordinateSet";

interface IProps {
    board: BoardArray;
    selectedTile: CoordinateTuple | null;
    currentTurnTiles: CoordinateSet;
    onDropFromRack: (row: number, column: number) => void;
    onSelect: (coordinates: CoordinateTuple) => void;
}

export const Board: React.FC<IProps> = ({
    board, selectedTile, currentTurnTiles, onDropFromRack, onSelect,
}) => {

    return (
        <div className={ styles.outer }>
            {/* Start arrow  */}
            <span className={ styles.start }>Start</span>
            <span className={ styles.arrow }>→</span>
            <table className={ styles.board }>
                <tbody>
                    { board.map((row, i) => (
                        <tr key={ i }>
                            { row.map((cell, j) => <BoardCell key={ `${i}-${j}` }
                                cell={ cell }
                                coordinates={ [i, j] }
                                boardWidth={ board[0].length - 1 }
                                selectedTile={ selectedTile }
                                currentTurnTiles={ currentTurnTiles }
                                onDropFromRack={ onDropFromRack }
                                onSelect={ onSelect }
                            />) }
                        </tr>
                    )) }
                </tbody>
            </table>
        </div>
    );
};
Board.displayName = "Board";

interface IBoardCellProps {
    cell: Cell;
    coordinates: CoordinateTuple;
    boardWidth: number;
    selectedTile: CoordinateTuple | null;
    currentTurnTiles: CoordinateSet;
    onDropFromRack: (row: number, column: number) => void;
    onSelect: (coordinates: CoordinateTuple) => void;
}

const BoardCell: React.FC<IBoardCellProps> = ({
    cell, coordinates, boardWidth, selectedTile, currentTurnTiles,
    onDropFromRack, onSelect,
}) => {
    const [i, j] = coordinates;
    const isEndGame = j === boardWidth;
    const type = isEndGame
        ? TileType.EndGame
        : cell.bonus > 0 ? TileType.Bonus
        : cell.bonus < 0 ? TileType.Penalty
        : TileType.Normal;
    const isFromCurrentTurn= currentTurnTiles.has(coordinates);
    const isSelected =  selectedTile !== null
        && i === selectedTile[0] && j === selectedTile[1];
    return (
        <td key={ j }>
            { cell.tilePlacement
            ? <TileCell tilePath={ cell.tilePlacement.tilePath }
                isUniversal={ cell.tilePlacement.isUniversal }
                isSelected={ isSelected }
                rotation={ cell.tilePlacement.rotation }
                type={ type }
                isFromCurrentTurn={ isFromCurrentTurn }
                onSelect={ () => onSelect(coordinates) }
            />
            : <EmptyCell bonus={ cell.bonus }
                isEndGame={ isEndGame }
                onDrop={ () => onDropFromRack(i, j) }
            /> }
        </td>
    );
};
BoardCell.displayName = "BoardCell";
