import { AddFilled16, Subtract16 } from "@carbon/icons-react";
import { Button } from "components/Button";
import { Game } from "components/Game";
import { PlayerNameInput } from "components/PlayerNameInput";
import React from "react";
import styles from "./App.module.css";

interface IState {
    playerNames: string[];
    hasConfirmed: boolean;
    cpuPlayerCount: number;
}

export const App: React.FC = () => {
    // State
    const [state, setState] = React.useState<IState>({
        playerNames: [""],
        hasConfirmed: false,
        cpuPlayerCount: 0
        // playerNames: ["player1"],
        // hasConfirmed: true,
        // cpuPlayerCount: 3,
    });
    if(state.hasConfirmed) {
        return (
            <Game playerNames={ state.playerNames } cpuPlayerCount={ state.cpuPlayerCount } />
        );
    }

    const onChange = (text: string, idx: number) => {
        setState((prevState) => ({
            hasConfirmed: prevState.hasConfirmed,
            playerNames: prevState.playerNames.map((name, i) => i === idx ? text : name),
            cpuPlayerCount: prevState.cpuPlayerCount,
        }));
    }

    const playerCount = state.playerNames.length + state.cpuPlayerCount;
    const canStart = playerCount >= 2 && playerCount <= 4;
    return (
        <>
            <h1>Nile</h1>
            <h2 className={ styles.centerText }>New game</h2>
            <form className={ styles.centerText }>
                { state.playerNames.map((name, i) => (
                    <PlayerNameInput key={ i }
                        i={ i }
                        name={ name }
                        onChange={ (t) => onChange(t, i) }
                    />
                )) }
                <Button title="Add player"
                    onClick={ () => setState((prevState) => ({...prevState, playerNames: [...prevState.playerNames, ""]})) }
                    enabled={ playerCount < 4 }
                >
                    <AddFilled16 aria-label="Add player" />
                </Button>
                <Button title="Remove player"
                    // Remove last player
                    onClick={ () => setState((prevState) => ({...prevState, playerNames: prevState.playerNames.filter((_, i) => i !== prevState.playerNames.length - 1)})) }
                    enabled={ playerCount > 1 }
                >
                    <Subtract16 aria-label="Remove player" />
                </Button>
                <br/>
                <span className={ styles.cpuCount }>CPU players: { state.cpuPlayerCount }</span>
                <Button title="Add CPU player"
                    onClick={ () => setState((prevState) => ({...prevState, cpuPlayerCount: prevState.cpuPlayerCount + 1})) }
                    enabled={ playerCount < 4 }
                >
                    <AddFilled16 aria-label="Add CPU player" />
                </Button>
                <Button title="Remove CPU player"
                    onClick={ () => setState((prevState) => ({...prevState, cpuPlayerCount: prevState.cpuPlayerCount - 1})) }
                    enabled={ playerCount > 1 }
                >
                    <Subtract16 aria-label="Remove CPU player" />
                </Button>
                <br/>
                <Button title={ canStart ? "Start new game" : "Need at least two players" }
                    onClick={ () => setState((prevState) => ({...prevState, hasConfirmed: true})) }
                    enabled={ canStart }
                >
                    Start
                </Button>
            </form>
        </>
    );
};
App.displayName = "App";
