import { PlayerData, sumTurnScores } from "lib/common";
import { Tile } from "nile";
import React from "react";
import { TileRack } from "./TileRack";
import styles from "./Player.module.css";

interface IProps {
    player: PlayerData;
    id: number;
    isCurrentTurn: boolean;
    isCpu: boolean;
    isExpanded: boolean;
    setDraggedTile: (idx: number, tile: Tile) => void;
}

export const Player: React.FC<IProps> = ({player, id, isCurrentTurn, isCpu, isExpanded, setDraggedTile}) => {
    const currentTurnScoreFwd = sumTurnScores(player.scores);
    let scoreFwd = 0;
    return (
        <section
            style={ {gridColumn: id + 1} }
        >
            <h2 className={ isCurrentTurn ? styles.current : styles.other }>
                { player.name }
            </h2>
            <TileRack tiles={ player.tileRack }
                showTiles={ isCurrentTurn && !isCpu }
                setDraggedTile={ setDraggedTile }
            />
            <table className={ styles.scores }>
                <thead>
                    <tr>
                        <th>Score Fwd</th>
                        <th>+</th>
                        <th>-</th>
                        <th>Net</th>
                    </tr>
                </thead>
                <tbody>
                    { isExpanded && player.scores.map((score, i) => {
                        const newScoreFwd = scoreFwd + score.add - score.sub;
                        const row = (
                            <tr key={ i }>
                                <td>{ scoreFwd }</td>
                                <td>{ score.add }</td>
                                <td>{ score.sub }</td>
                                <td>{ newScoreFwd }</td>
                            </tr>
                        );
                        scoreFwd = newScoreFwd;
                        return row;
                    }) }
                    <tr key={ player.scores.length }>
                        <td>
                            { currentTurnScoreFwd }
                        </td>
                        {/* Only display current turn scores during turn */}
                        <td>{ isCurrentTurn ? player.currentTurnScore.add : null }</td>
                        <td>{ isCurrentTurn ? player.currentTurnScore.sub : null }</td>
                        <td>{ isCurrentTurn ? currentTurnScoreFwd + player.currentTurnScore.add - player.currentTurnScore.sub : null }</td>
                    </tr>
                </tbody>
            </table>
        </section>
    );
}
Player.displayName = "Player";
