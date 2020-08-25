import { PlayerData } from "lib/common";
import { Tile } from "nile";
import React from "react";
import { TileRack } from "./TileRack";
import styles from "./Player.module.css";

interface IProps {
    player: PlayerData;
    isCurrentTurn: boolean;
    setDraggedTile: (idx: number, tile: Tile) => void;
}

export const Player: React.FC<IProps> = ({player, isCurrentTurn, setDraggedTile}) => {
    const scoreFwd = player.scores.reduce((acc, score) => (
        acc + score.add + score.sub
    ), 0);
    return (
        <li key={ player.name }>
            <h2>{ player.name }</h2>
            { isCurrentTurn && <TileRack tiles={ player.tileRack }
                isCurrentTurn={ isCurrentTurn }
                setDraggedTile={ setDraggedTile }
            /> }
            <table className={ styles.scores }>
                <thead>
                    <tr>
                        <th></th>
                        <th>Score Fwd</th>
                        <th>+</th>
                        <th>-</th>
                        <th>Net</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td>Current turn</td>
                        <td>
                            { scoreFwd }
                        </td>
                        <td>{ player.currentTurnScore.add }</td>
                        <td>{ player.currentTurnScore.sub }</td>
                        <td>{ scoreFwd + player.currentTurnScore.add + player.currentTurnScore.sub }</td>
                    </tr>
                </tbody>
            </table>
        </li>
    );
}
Player.displayName = "Player";
