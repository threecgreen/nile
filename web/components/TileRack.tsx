import { Tile as TileEnum } from "nile";
import React from "react";
import { GridCell } from "./Grid";
import { RackTile } from "./Tile";

interface IProps {
    tiles: TileEnum[];
    isCurrentTurn: boolean;
    setDraggedTile: (idx: number, tile: TileEnum) => void;
}

export const TileRack: React.FC<IProps> = ({tiles, isCurrentTurn, setDraggedTile}) => {
    const onDrag = (e: React.DragEvent) => {
        e.preventDefault();
    }

    const onClick = (e: React.MouseEvent, i: number, tile: TileEnum) => {
        e.preventDefault();
        setDraggedTile(i, tile);
    }

    return (
        <table>
            <tbody>
                <tr>
                    { tiles.map((tile, i) => (
                        <GridCell key={ `${tile} - ${i}` }>
                            <div draggable={ isCurrentTurn }
                                onDrag={ onDrag }
                                onDragStart={ (_) => setDraggedTile(i, tile) }
                                onTouchStart={ (_) => setDraggedTile(i, tile) }
                                // TODO: how to show visually
                                onClick={ (e) => onClick(e, i, tile) }
                            >
                                <RackTile tile={ tile } />
                            </div>
                        </GridCell>
                    )) }
                </tr>
            </tbody>
        </table>
    );
}
TileRack.displayName = "TileRack";
