import React from "react";
import { Game } from "components/Game";
import { PlayerNameInput } from "components/PlayerNameInput";

interface IState {
    playerNames: string[];
    hasConfirmed: boolean;
}

export const App: React.FC = () => {
    // State
    const [state, setState] = React.useState<IState>({
        // playerNames: [""],
        // hasConfirmed: false,
        playerNames: ["player1", "player2"],
        hasConfirmed: true,
    });
    if(state.hasConfirmed) {
        return (
            <Game playerNames={ state.playerNames } />
        );
    }

    const onChange = (text: string, idx: number) => {
        setState((prevState) => ({
            hasConfirmed: prevState.hasConfirmed,
            playerNames: prevState.playerNames.map((name, i) => i === idx ? text : name),
        }));
    }

    return (
        <form>
            { state.playerNames.map((name, i) => (
                <PlayerNameInput key={ i }
                    i={ i }
                    name={ name }
                    onChange={ (t) => onChange(t, i) }
                />
            ))}
        </form>
    );
};
App.displayName = "App";
