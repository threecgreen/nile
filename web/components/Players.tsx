import { PlayerData } from "lib/common";
import { Tile, TilePath, TilePathType } from "nile";
import React from "react";
import { Player } from "./Player";
import styles from "./Players.module.css";

interface IProps {
    currentPlayerId: number;
    playerData: PlayerData[];
    setDraggedTile: (isUniversal: boolean, tilePath: TilePath, idx: number) => void;
}

export const Players: React.FC<IProps> = ({currentPlayerId, playerData, setDraggedTile}) => {
    return (
        <div className={ styles.players }>
            <div style={ {columnCount: playerData.length} }>
                { playerData.map((player, id) => (
                    <Player player={ player }
                        key={ player.name }
                        id={ id }
                        isCurrentTurn={ id === currentPlayerId }
                        isCpu={ player.isCpu }
                        setDraggedTile={ (idx, tile) => {
                            if (tile === Tile.Universal) {
                                // Default TilePath for now
                                setDraggedTile(true, TilePath.Straight, idx);
                            } else {
                                const tpt = TilePathType.tile_into_normal(tile);
                                setDraggedTile(false, tpt.tile_path(), idx);
                            }
                        } }
                    />
                )) }
            </div>
        </div>
    );
}
Players.displayName = "Players";
