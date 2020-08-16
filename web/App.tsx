import React from "react";
import { Tile as TileEnum, Rotation } from "nile";
import { Tile } from "components/Tile";

export const App: React.FC = () => {
    return (
        <div>
            <Tile tile={ TileEnum.Straight } rotation={ Rotation.Clockwise90 } />
            <Tile tile={ TileEnum.Diagonal } rotation={ Rotation.Clockwise90 } />
            <Tile tile={ TileEnum.Center90 } rotation={ Rotation.Clockwise90 } />
            <Tile tile={ TileEnum.Corner90 } rotation={ Rotation.Clockwise90 } />
            <Tile tile={ TileEnum.Left45 } rotation={ Rotation.Clockwise180 } />
            <Tile tile={ TileEnum.Right45 } rotation={ Rotation.None } />
            <Tile tile={ TileEnum.Left135 } rotation={ Rotation.None } />
            <Tile tile={ TileEnum.Right135 } rotation={ Rotation.None } />
            <Tile tile={ TileEnum.Universal } rotation={ Rotation.None } />
        </div>
    );
};

