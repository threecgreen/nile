import { DownToBottom24, Redo24, RotateClockwise24, RotateCounterclockwise24, Undo24, Checkmark24, Close24, TrashCan24 } from "@carbon/icons-react";
import { c } from "lib/utils";
import { TilePath, tile_path_to_tile } from "nile";
import React from "react";
import { Button } from "./Button";
import styles from "./Controls.module.css";
import { RackTile } from "./Tile";

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
    onCantPlay: () => void;
}

export const Controls: React.FC<IProps> = ({
    hasPlacedTile, hasSelectedTile, selectedIsUniversal,  canUndo, canRedo,
    onRotate, onRemoveTile, onUpdateUniversalPath, onUndo, onRedo, onEndTurn, onCantPlay,
}) => {
    const [isTilePathSelectorOpen, setIsTilePathSelectorOpen] = React.useState(false);
    React.useEffect(() => {
        if(!selectedIsUniversal && isTilePathSelectorOpen) {
            setIsTilePathSelectorOpen(false);
        }
    }, [isTilePathSelectorOpen, selectedIsUniversal]);
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
                title="Remove tile"
            >
                <TrashCan24 aria-label="Remove tile" />
                Remove tile
            </Button>
            <div className={ c([styles.dropdown, selectedIsUniversal ? "" : "disabled"]) }>
                <Button enabled={ selectedIsUniversal }
                    className={ styles.dropdown }
                    onClick={ () => setIsTilePathSelectorOpen(!isTilePathSelectorOpen) }
                >
                    Tile Path <DownToBottom24 />
                </Button>
                { isTilePathSelectorOpen &&
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
                </div> }
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
                <Checkmark24 aria-label="End turn" />
                End Turn
            </Button>
            <Button
                enabled={ !hasPlacedTile }
                onClick={ onCantPlay }
            >
                <Close24 aria-label="Can't play" />
                Can&apos;t play
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
            <RackTile tile={ tile_path_to_tile(tilePath) } isSelected={ false } />
        </a>
    );
}
TilePathSelection.displayName = "TilePathSelection";
