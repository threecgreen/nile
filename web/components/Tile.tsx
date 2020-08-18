import React from "react";
import { Tile as TileEnum, Rotation } from "nile";
import Straight from "assets/tiles/Straight.svg"
import Diagonal from "assets/tiles/Diagonal.svg"
import Center90 from "assets/tiles/Center90.svg"
import Corner90 from "assets/tiles/Corner90.svg"
import Tile45 from "assets/tiles/45.svg";
import Tile135 from "assets/tiles/135.svg";
import Universal from "assets/tiles/Universal.svg";
import styles from "components/Tile.module.css";

interface IProps {
    tile: TileEnum,
    rotation: Rotation,
};

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

export const Tile: React.FC<IProps> = ({tile, rotation}) => {
    let svg;
    let reflect = false;
    switch (tile) {
        case TileEnum.Straight:
            svg = <Straight />;
            break;
        case TileEnum.Diagonal:
            svg = <Diagonal />;
            break;
        case TileEnum.Center90:
            svg = <Center90 />;
            break;
        case TileEnum.Corner90:
            svg = <Corner90 />;
            break;
        case TileEnum.Left45:
            svg = <Tile45 />;
            reflect = true;
            break;
        case TileEnum.Right45:
            svg = <Tile45 />;
            break;
        case TileEnum.Left135:
            svg = <Tile135 />;
            reflect = true;
            break;
        case TileEnum.Right135:
            svg = <Tile135 />;
            break;
        case TileEnum.Universal:
            svg = <Universal />;
            break;
        default:
            throw new Error("Unknown tile type");
    }
    return (
        <div className={ styles.tile }
            style={ { transform: `${rotationToCSs(rotation)} ${reflectToCss(reflect)}`} }
        >
            { svg }
        </div>
    );
}
Tile.displayName = "Tile";

export const EmptyTile: React.FC<{onDrop: () => void}> = (props) => {
    const onDrop = (e: React.DragEvent) => {
        e.preventDefault();
        props.onDrop();
    }
    return (
        <div className={ styles.tile }
            // Allow tiles to be dropped here
            onDragOver={ (e) => e.preventDefault() }
            onDrop={ onDrop }
        />
    );
}
EmptyTile.displayName = "EmptyTile";
