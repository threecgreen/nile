import { DownToBottom20, Redo24, RotateClockwise24, RotateCounterclockwise24, Undo24 } from "@carbon/icons-react";
import { TilePath, tile_path_to_tile } from "nile";
import React from "react";
import { Button } from "./Button";
import styles from "./Controls.module.css";
import { TileSvg } from "./TileSvg";
import { RackTile } from "./Tile";
import { c } from "lib/utils";

interface IProps {
    hasPlacedTile: boolean;
    hasSelectedTile: boolean;
    selectedIsUniversal: boolean;
    canUndo: boolean;
    canRedo: boolean;
    onRotate: (isClockwise: boolean) => void;
    onRemoveTile: () => void;
    onUpdateUniversalPath: (tilePath: TilePath) => void;
    onUndo: () => void;
    onRedo: () => void;
    onEndTurn: () => void;
}

export const Controls: React.FC<IProps> = ({
    hasPlacedTile, hasSelectedTile, selectedIsUniversal,  canUndo, canRedo,
    onRotate, onRemoveTile, onUpdateUniversalPath, onUndo, onRedo, onEndTurn
}) => {
    return (
        <div className={ styles.controls }>
            <Button enabled={ hasSelectedTile }
                onClick={ () => onRotate(false) }
                title="Rotate tile counter-clockwise"
            >
                <RotateCounterclockwise24 aria-label="Rotate counter-clockwise" />
            </Button>
            <Button enabled={ hasSelectedTile }
                onClick={ () => onRotate(true) }
                title="Rotate tile clockwise"
            >
                <RotateClockwise24 aria-label="Rotate clockwise" />
            </Button>
            <Button enabled={ hasPlacedTile }
                onClick={ onRemoveTile }
            >
                {/* <FilterRemove24 aria-label="Remove tile" /> */}
                Remove tile
            </Button>
            <div className={ c([styles.dropdown, selectedIsUniversal ? "" : "disabled"]) }>
                <Button enabled={ selectedIsUniversal }
                    className={ styles.dropdown }
                    onClick={ () => undefined }
                >
                    Tile Path <DownToBottom20 />
                </Button>
                <div className={ styles.dropdownContent }>
                    <TilePathSelection tilePath={ TilePath.Straight }
                        onUpdateUniversalPath={ onUpdateUniversalPath }
                    />
                    <TilePathSelection tilePath={ TilePath.Diagonal }
                        onUpdateUniversalPath={ onUpdateUniversalPath }
                    />
                    <TilePathSelection tilePath={ TilePath.Center90 }
                        onUpdateUniversalPath={ onUpdateUniversalPath }
                    />
                    <TilePathSelection tilePath={ TilePath.Corner90 }
                        onUpdateUniversalPath={ onUpdateUniversalPath }
                    />
                    <TilePathSelection tilePath={ TilePath.Left45 }
                        onUpdateUniversalPath={ onUpdateUniversalPath }
                    />
                    <TilePathSelection tilePath={ TilePath.Right45 }
                        onUpdateUniversalPath={ onUpdateUniversalPath }
                    />
                    <TilePathSelection tilePath={ TilePath.Left135 }
                        onUpdateUniversalPath={ onUpdateUniversalPath }
                    />
                    <TilePathSelection tilePath={ TilePath.Right135 }
                        onUpdateUniversalPath={ onUpdateUniversalPath }
                    />
                </div>
            </div>
            <Button enabled={ canUndo }
                onClick={ onUndo }
                title="Undo"
            >
                <Undo24 aria-label="undo" />
            </Button>
            <Button enabled={ canRedo }
                onClick={ onRedo }
                title="Redo"
            >
                <Redo24 aria-label="redo" />
            </Button>
            <Button
                // Must have played at least one tile
                enabled={ hasPlacedTile }
                onClick={ onEndTurn }
            >
                End Turn
            </Button>
        </div>
    );
}
Controls.displayName = "Controls";

const TilePathSelection: React.FC<{tilePath: TilePath, onUpdateUniversalPath: (tilePath: TilePath) => void}> = ({tilePath, onUpdateUniversalPath}) => {
    const onClick = (e: React.MouseEvent) => {
        e.preventDefault();
        onUpdateUniversalPath(tilePath);
    }

    return (
        <a onClick={ onClick }>
            <RackTile tile={ tile_path_to_tile(tilePath) } />
        </a>
    );
}
TilePathSelection.displayName = "TilePathSelection";