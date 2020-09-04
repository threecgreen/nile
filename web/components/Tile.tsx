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

export const RackTile: React.FC<{tile: TileEnum}> = ({tile}) => (
    <div className={ styles.tile }>
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
    onSelect: () => void;
};

export const Tile: React.FC<IProps> = ({
    tilePath, isUniversal, rotation, isSelected, type,
    ...props
}) => {
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
                tileTypeToClass[type],
                styles.hasTile
            ]) }
            style={ {transform: rotationToCSs(rotation)} }
            onClick={ onSelect }
        >
            {
                isUniversal
                && <TileSvg tile={ TileEnum.Universal }
                    strokeColor="#888888"
                />
            }
            <TileSvg tile={ tile_path_to_tile(tilePath) }
                strokeColor="royalblue"
            />
        </div>
    );
}
Tile.displayName = "Tile";

interface IEmptyTileProps {
    bonus: number;
    isEndGame: boolean;
    onDrop: () => void;
}

export const EmptyTile: React.FC<IEmptyTileProps> = ({bonus, isEndGame, ...props}) => {
    const onDrop = (e: React.DragEvent) => {
        e.preventDefault();
        props.onDrop();
    }
    const onTouchEnd = (e: React.TouchEvent) => {
        e.preventDefault();
        props.onDrop();
    }

    return (
        <div className={ c([styles.tile, bonusToClassName(bonus), isEndGame ? styles.endGame : undefined]) }
            // Allow tiles to be dropped here
            onDragOver={ (e) => e.preventDefault() }
            onTouchEnd={ onTouchEnd }
            onDrop={ onDrop }
        >
            { bonus ? <p>{ Math.abs(bonus) }</p> : null }
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
