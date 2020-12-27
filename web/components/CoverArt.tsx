import { Rotation, TilePath } from "nile";
import React from "react";
import { DisplayTile } from "./Tile";

export const CoverArt: React.FC = () => (
    <>
        <DisplayTile tilePath={ TilePath.Center90 }
            isUniversal={ false }
            rotation={ Rotation.None }
        />
        <DisplayTile tilePath={ TilePath.Right45 }
            isUniversal={ true }
            rotation={ Rotation.Clockwise90 }
        />
        <DisplayTile tilePath={ TilePath.Corner90 }
            isUniversal={ false }
            rotation={ Rotation.Clockwise180 }
        />
        <DisplayTile tilePath={ TilePath.Corner90 }
            isUniversal={ false }
            rotation={ Rotation.None }
        />
        <DisplayTile tilePath={ TilePath.Right135 }
            isUniversal={ false }
            rotation={ Rotation.Clockwise180 }
        />
        <DisplayTile tilePath={ TilePath.Center90 }
            isUniversal={ false }
            rotation={ Rotation.None }
        />
    </>
);
CoverArt.displayName = "CoverArt";
