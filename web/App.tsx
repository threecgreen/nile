import React from "react";
import { Game } from "components/Game";
import { PlayerNameInput } from "components/PlayerNameInput";
import { AddFilled24, Subtract24, Subtract16, AddFilled16 } from "@carbon/icons-react";
import { Button } from "components/Button";

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
        // playerNames: ["player1"],
        // hasConfirmed: true,
        cpuPlayerCount: 3,
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
    return (
        <form>
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
            CPU players: { state.cpuPlayerCount }
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
            <Button title="Start"
                onClick={ () => setState((prevState) => ({...prevState, hasConfirmed: true})) }
            >
                Start
            </Button>
        </form>
    );
};
App.displayName = "App";
