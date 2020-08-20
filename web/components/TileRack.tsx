import { Rotation, Tile as TileEnum } from "nile";
import React from "react";
import { Grid, GridCell } from "./Grid";
import { Tile } from "./Tile";

interface IProps {
    tiles: TileEnum[];
    isCurrentTurn: boolean;
    onDrag: (tile: TileEnum) => void;
}

export const TileRack: React.FC<IProps> = ({tiles, isCurrentTurn, ...props}) => {
    const onDrag = (e: React.DragEvent) => {
        e.preventDefault();
    }

    const onDragStart = (_: React.DragEvent, tile: TileEnum) => {
        props.onDrag(tile);
    }

    return (
        <Grid width={ tiles.length * 41 + 1 }>
            { tiles.map((tile, i) => (
                <GridCell key={ `${tile} - ${i}` }
                    column={ i }
                    row={ 0 }
                >
                    <div draggable={ isCurrentTurn }
                        onDrag={ onDrag }
                        onDragStart={ (e) => onDragStart(e, tile) }
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
        </Grid>
    );
}
TileRack.displayName = "TileRack";
