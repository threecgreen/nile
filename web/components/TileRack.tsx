import { Tile as TileEnum } from "nile";
import React from "react";
import { GridCell } from "./Grid";
import { HiddenTile, RackTile } from "./Tile";

interface IProps {
    tiles: TileEnum[];
    showTiles: boolean;
    selectedTileIdx: number | null;
    onSelect: (idx: number, tile: TileEnum) => void;
}

export const TileRack: React.FC<IProps> = ({tiles, showTiles, selectedTileIdx, onSelect}) => {
    const onDrag = (e: React.DragEvent) => {
        e.preventDefault();
    }

    const onClick = (e: React.MouseEvent, i: number, tile: TileEnum) => {
        e.preventDefault();
        onSelect(i, tile);
    }

    return (
        <table>
            <tbody>
                <tr>
                    { tiles.map((tile, i) => (
                        <GridCell key={ `${tile} - ${i}` }>
                            <div draggable={ showTiles }
                                onDrag={ onDrag }
                                onDragStart={ (_) => onSelect(i, tile) }
                                onTouchStart={ (_) => onSelect(i, tile) }
                                // TODO: how to show visually
                                onClick={ (e) => onClick(e, i, tile) }
                            >
                                { showTiles
                                    ? <RackTile tile={ tile }
                                        isSelected={ i === selectedTileIdx }
                                    />
                                    : <HiddenTile /> }
                            </div>
                        </GridCell>
                    )) }
                </tr>
            </tbody>
        </table>
    );
}
TileRack.displayName = "TileRack";
