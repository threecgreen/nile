import { c } from "lib/utils";
import { Rotation, Tile as TileEnum, TilePath, tile_path_to_tile } from "nile";
import React from "react";
import styles from "./Tile.module.css";
import { TileSvg } from "./TileSvg";

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

export const RackTile: React.FC<{tile: TileEnum, isSelected: boolean}> = ({tile, isSelected}) => (
    <div className={ c([styles.tile, isSelected ? styles.selected : undefined]) }>
        <TileSvg tile={ tile } />
    </div>
)
RackTile.displayName = "RackTile";

export enum TileType {
    Normal,
    Bonus,
    Penalty,
    EndGame,
}

const tileTypeToClass = {
    [TileType.Normal]: undefined,
    [TileType.Bonus]: styles.bonus,
    [TileType.Penalty]: styles.penalty,
    [TileType.EndGame]: styles.endGame,
};

interface IProps {
    tilePath: TilePath;
    isUniversal: boolean;
    rotation: Rotation;
    isSelected: boolean;
    type: TileType,
    isFromCurrentTile: boolean;
    onSelect: () => void;
}

export const TileCell: React.FC<IProps> = ({
    tilePath, isUniversal, rotation, isSelected, type, isFromCurrentTile,
    ...props
}) => {
    const onSelect = (e: React.MouseEvent) => {
        e.preventDefault();
        props.onSelect();
    }
    const onDrag = (e: React.DragEvent) => {
        e.preventDefault();
    }

    return (
        <div
            className={ c([
                styles.tile,
                isSelected ? styles.selected : undefined,
                isUniversal ? styles.universal : undefined,
                tileTypeToClass[type],
                styles.hasTile,
            ]) }
            style={ {transform: rotationToCSs(rotation)} }
            onClick={ isFromCurrentTile ? onSelect : undefined }
            draggable={ isFromCurrentTile }
            onDrag={ onDrag }
            onDragStart={ props.onSelect }
        >
            {
                isUniversal
                && <TileSvg tile={ TileEnum.Universal }
                    strokeColor="#aaaaaa"
                />
            }
            <TileSvg tile={ tile_path_to_tile(tilePath) }
                strokeColor="royalblue"
            />
        </div>
    );
}
TileCell.displayName = "TileCell";

interface IEmptyTileProps {
    bonus: number;
    isEndGame: boolean;
    onDrop: () => void;
}

export const EmptyCell: React.FC<IEmptyTileProps> = ({bonus, isEndGame, ...props}) => {
    const onDrop = (e: React.DragEvent) => {
        e.preventDefault();
        props.onDrop();
    }
    const onClick = (e: React.MouseEvent) => {
        e.preventDefault();
        props.onDrop();
    }

    return (
        <div className={ c([styles.tile, bonusToClassName(bonus), isEndGame ? styles.endGame : undefined]) }
            // Allow tiles to be dropped here
            onDragOver={ (e) => e.preventDefault() }
            onDrop={ onDrop }
            onClick={ onClick }
        >
            { bonus ? <p>{ Math.abs(bonus) }</p> : null }
        </div>
    );
}
EmptyCell.displayName = "EmptyCell";

const bonusToClassName = (bonus: number): string | undefined => {
    if (bonus > 0) {
        return styles.bonus;
    }
    if (bonus < 0) {
        return styles.penalty;
    }
    return undefined;
}

export const HiddenTile: React.FC = ({}) => (
    <div className={ c([styles.tile, styles.hiddenTile]) }></div>
)
HiddenTile.displayName = "HiddenTile";
