import React from "react";
import { PlayerData } from "lib/common";

interface IProps {
    currentPlayerId: number;
    playerData: PlayerData[];
}

// TODO: make expandable to view full history
export const Scoreboard: React.FC<IProps> = ({currentPlayerId, playerData}) => {
    let c = currentPlayerId;
    return (
        <table>
            <thead>
                <tr>
                    <td>Player</td>
                    <td>Score Fwd</td>
                    <td>+</td>
                    <td>-</td>
                    <td>Net</td>
                </tr>
            </thead>
            <tbody>
                { playerData.map((player) => {
                    // TODO: color current player differently
                    const scoreFwd = player.scores.reduce((acc, score) => (
                        acc + score.add - score.sub
                    ), 0);
                    return (
                        <tr>
                            <td>{ player.name }</td>
                            <td>{ scoreFwd }</td>
                            <td>{ player.currentTurnScore.sub }</td>
                            <td>{ player.currentTurnScore.add }</td>
                            <td>{ scoreFwd + player.currentTurnScore.add - player.currentTurnScore.sub }</td>
                        </tr>
                    );
                }) }
            </tbody>
        </table>
    )
}
Scoreboard.displayName = "Scoreboard";
