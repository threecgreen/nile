import { Rotation, Tile as TileEnum } from "nile";
import React from "react";
import { GridCell } from "./Grid";
import { Tile } from "./Tile";

interface IProps {
    tiles: TileEnum[];
    isCurrentTurn: boolean;
    setDraggedTile: (idx: number, tile: TileEnum) => void;
}

export const TileRack: React.FC<IProps> = ({tiles, isCurrentTurn, setDraggedTile}) => {
    const onDrag = (e: React.DragEvent) => {
        e.preventDefault();
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
                            >
                                <Tile rotation={ Rotation.None }
                                    tile={ tile }
                                    // TODO: break up this functionality
                                    isSelected={ false }
                                    onSelect={ () => undefined }
                                />
                            </div>
                        </GridCell>
                    )) }
                </tr>
            </tbody>
        </table>
    );
}
TileRack.displayName = "TileRack";
