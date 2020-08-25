import React from "react";
import { Button } from "./Button";
import { RotateCounterclockwise24, RotateClockwise24, Undo24, Redo24 } from "@carbon/icons-react";
import styles from "./Controls.module.css";

interface IProps {
    hasPlacedTile: boolean;
    hasSelectedTile: boolean;
    canUndo: boolean;
    canRedo: boolean;
    onRotate: (isClockwise: boolean) => void;
    onRemoveTile: () => void;
    onUndo: () => void;
    onRedo: () => void;
    onEndTurn: () => void;
}

export const Controls: React.FC<IProps> = ({
    hasPlacedTile, hasSelectedTile, canUndo, canRedo, onRotate, onRemoveTile, onUndo, onRedo, onEndTurn
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