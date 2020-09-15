import { Container } from "components/Container";
import { Game } from "components/Game";
import { GameForm } from "components/GameForm";
import React from "react";
import { Button } from "components/Button";

interface IState {
    playerNames: string[];
    hasConfirmed: boolean;
    cpuPlayerCount: number;
    gameNumber: number;
}

export const App: React.FC = () => {
    const [state, setState] = React.useState<IState>({
        playerNames: [""],
        hasConfirmed: false,
        cpuPlayerCount: 1,
        gameNumber: 1,
    });

    const onChange = (text: string, idx: number) => {
        setState((prevState) => ({
            ...prevState,
            playerNames: prevState.playerNames.map((name, i) => i === idx ? text : name),
        }));
    }

    // onNewGame={}
    return (
        <Container>
            <h1>Nile</h1>
            { state.hasConfirmed
                // Incrementing key will remount Game
                ? <>
                    {/* TODO: Color green */}
                    <Button title="New game"
                        onClick={ () => setState((prevState) => ({...prevState, hasConfirmed: false, gameNumber: prevState.gameNumber + 1})) }
                    >
                        New game
                    </Button>
                    <Game key={ state.gameNumber }
                    playerNames={ state.playerNames }
                    cpuPlayerCount={ state.cpuPlayerCount }
                    />
                </>
                : <>
                    <h2 className="centerText">New game</h2>
                    <GameForm playerNames={ state.playerNames }
                        cpuPlayerCount={ state.cpuPlayerCount }
                        onPlayerNameChange={ onChange }
                        onAddPlayer={ () => setState((prevState) => ({...prevState, playerNames: [...prevState.playerNames, ""]})) }
                        // Remove last player
                        onRemovePlayer={ () => setState((prevState) => ({...prevState, playerNames: prevState.playerNames.filter((_, i) => i !== prevState.playerNames.length - 1)}))  }
                        onAddCpuPlayer={ () => setState((prevState) => ({...prevState, cpuPlayerCount: prevState.cpuPlayerCount + 1})) }
                        onRemoveCpuPlayer={ () => setState((prevState) => ({...prevState, cpuPlayerCount: prevState.cpuPlayerCount - 1})) }
                        onClickStart={ () => setState((prevState) => ({...prevState, hasConfirmed: true})) }
                    />
                </> }
        </Container>
    );
};
App.displayName = "App";
