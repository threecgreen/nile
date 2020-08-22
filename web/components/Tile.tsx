import React from "react";
import { Tile as TileEnum, Rotation, TilePath, tile_path_to_tile } from "nile";
import Straight from "assets/tiles/Straight.svg"
import Diagonal from "assets/tiles/Diagonal.svg"
import Center90 from "assets/tiles/Center90.svg"
import Corner90 from "assets/tiles/Corner90.svg"
import Tile45 from "assets/tiles/45.svg";
import Tile135 from "assets/tiles/135.svg";
import Universal from "assets/tiles/Universal.svg";
import styles from "components/Tile.module.css";
import { c } from "lib/utils";

const rotationToCSs = (rotation: Rotation): string => {
    switch (rotation) {
        case Rotation.Clockwise90:
            return "rotate(90deg)";
        case Rotation.Clockwise180:
            return "rotate(180deg)";
        case Rotation.Clockwise270:
            return "rotate(270deg)";
        default:
            return "";
    }
}

const reflectToCss = (reflect: boolean): string => {
    return reflect ? "scaleX(-1)" : "";
}

export const RackTile: React.FC<{tile: TileEnum}> = ({tile}) => (
    <div className={ styles.tile }>
        <TileSvg tile={ tile } />
    </div>
)
RackTile.displayName = "RackTile";

const TileSvg: React.FC<{tile: TileEnum}> = ({tile}) => {
    switch (tile) {
        case TileEnum.Straight:
            return <Straight />;
        case TileEnum.Diagonal:
            return <Diagonal />;
        case TileEnum.Center90:
            return <Center90 />;
        case TileEnum.Corner90:
            return <Corner90 />;
        case TileEnum.Left45:
            return <Tile45
                style={ {transform: reflectToCss(true)} }
            />;
        case TileEnum.Right45:
            return <Tile45 />;
        case TileEnum.Left135:
            return <Tile135
                style={ {transform: reflectToCss(true)} }
            />;
        case TileEnum.Right135:
            return <Tile135 />;
        case TileEnum.Universal:
            return <Universal />;
        default:
            throw new Error(`Unknown tile type: ${tile}`);
    }
}
TileSvg.displayName = "TileSvg";

interface IProps {
    tilePath: TilePath;
    isUniversal: boolean;
    rotation: Rotation;
    isSelected: boolean;
    onSelect: () => void;
};

export const Tile: React.FC<IProps> = ({tilePath, isUniversal, rotation, isSelected, ...props}) => {
    const onSelect = (e: React.MouseEvent) => {
        e.preventDefault();
        props.onSelect();
    }

    return (
        <div
            className={ c([
                styles.tile,
                isSelected ? styles.selected : undefined,
                isUniversal ? styles.universal : undefined,
            ]) }
            style={ {transform: rotationToCSs(rotation)} }
            onClick={ onSelect }
        >
            <TileSvg tile={ tile_path_to_tile(tilePath) } />
            {/* TODO: place background universal tile if universal */}
        </div>
    );
}
Tile.displayName = "Tile";

interface IEmptyTileProps {
    bonus: number;
    onDrop: () => void;
}

export const EmptyTile: React.FC<IEmptyTileProps> = ({bonus, ...props}) => {
    const onDrop = (e: React.DragEvent) => {
        e.preventDefault();
        props.onDrop();
    }
    return (
        <div className={ c([styles.tile, bonusToClassName(bonus)]) }
            // Allow tiles to be dropped here
            onDragOver={ (e) => e.preventDefault() }
            onDrop={ onDrop }
        >
            { bonus ? Math.abs(bonus) : null }
        </div>
    );
}
EmptyTile.displayName = "EmptyTile";

const bonusToClassName = (bonus: number): string | undefined => {
    if (bonus > 0) {
        return styles.bonus;
    }
    if (bonus < 0) {
        return styles.penalty;
    }
    return undefined;
}
