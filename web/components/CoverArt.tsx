import { Rotation, TilePath } from "nile";
import React from "react";
import { DisplayTile } from "./Tile";
import styles from "./CoverArt.module.css";

export const CoverArt: React.FC = () => (
    <div>
        <DisplayTile tilePath={ TilePath.Center90 }
            isUniversal={ false }
            rotation={ Rotation.Clockwise270 }
        />
        <DisplayTile tilePath={ TilePath.Right45 }
            isUniversal={ true }
            rotation={ Rotation.Clockwise90 }
            className={ styles.negativeMargin }
        />
        <DisplayTile tilePath={ TilePath.Corner90 }
            isUniversal={ false }
            rotation={ Rotation.Clockwise180 }
            className={ styles.downRight }
        />
        <DisplayTile tilePath={ TilePath.Corner90 }
            isUniversal={ false }
            rotation={ Rotation.None }
        />
        <DisplayTile tilePath={ TilePath.Right135 }
            isUniversal={ false }
            rotation={ Rotation.Clockwise180 }
            className={ styles.downRight }
        />
        <DisplayTile tilePath={ TilePath.Center90 }
            isUniversal={ false }
            rotation={ Rotation.Clockwise270 }
            className={ styles.up }
        />
        <DisplayTile tilePath={ TilePath.Straight }
            isUniversal={ false }
            rotation={ Rotation.None }
            className={ styles.negativeMargin }
        />
        <DisplayTile tilePath={ TilePath.Straight }
            isUniversal={ false }
            rotation={ Rotation.None }
            className={ styles.negativeMargin }
        />
    </div>
);
CoverArt.displayName = "CoverArt";
