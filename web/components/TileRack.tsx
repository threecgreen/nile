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
    const onDrag = (e: React.DragEvent, tile: TileEnum) => {
        e.preventDefault();
        props.onDrag(tile);
    }

    return (
        <Grid width={ tiles.length * 41 + 1 }>
            { tiles.map((tile, i) => (
                <GridCell key={ i }
                    column={ i }
                    row={ 0 }
                >
                    <div draggable={ isCurrentTurn }
                        onDrag={ (e) => onDrag(e, tile) }
                    >
                        <Tile rotation={ Rotation.None }
                            tile={ tile }
                        />
                    </div>
                </GridCell>
            )) }
        </Grid>
    );
}
TileRack.displayName = "TileRack";
