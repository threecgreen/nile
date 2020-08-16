import React from "react";
import { Tile as TileEnum, Rotation } from "nile";
// @ts-ignore
import Straight from "../assets/tiles/Straight.svg"
// @ts-ignore
import Diagonal from "../assets/tiles/Diagonal.svg"
// @ts-ignore
import Center90 from "../assets/tiles/Center90.svg"
// @ts-ignore
import Corner90 from "../assets/tiles/Corner90.svg"
// @ts-ignore
import Tile45 from "../assets/tiles/45.svg";
// @ts-ignore
import Tile135 from "../assets/tiles/135.svg";
// @ts-ignore
import Universal from "../assets/tiles/Universal.svg";

interface IProps {
    tile: TileEnum,
    rotation: Rotation,
}

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
        <div style={ {
            height: "40px",
            width: "40px",
            border: "solid",
            borderWidth: "1px",
            marginTop: "-0.5px",
            marginBottom: "-0.5px",
            transform: `${rotationToCSs(rotation)} ${reflectToCss(reflect)}`} }
        >
            { svg }
        </div>
    )
}
