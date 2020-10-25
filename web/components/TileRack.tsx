import { score } from "lib/common";
import { Tile as TileEnum } from "nile";
import React from "react";
import { HiddenTile, RackTile } from "./Tile";
import styles from "./TileRack.module.css";

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
                        <td key={ `${tile} - ${i}` }>
                            <div draggable={ showTiles }
                                onDrag={ onDrag }
                                onDragStart={ () => onSelect(i, tile) }
                                onTouchStart={ () => onSelect(i, tile) }
                                // TODO: how to show visually
                                onClick={ (e) => onClick(e, i, tile) }
                            >
                                { showTiles
                                    ? <RackTile tile={ tile }
                                        isSelected={ i === selectedTileIdx }
                                    />
                                    : <HiddenTile /> }
                            </div>
                        </td>
                    )) }
                </tr>
                <tr className={ styles.alignRight }>
                    { tiles.map((tile, i) => (
                        <td key={ `${tile} - ${i}`}>
                            { showTiles && score(tile) }
                        </td>
                    ))}
                </tr>
            </tbody>
        </table>
    );
};
TileRack.displayName = "TileRack";
