import { Button } from "components/Button";
import { Container } from "components/Container";
import { CoverArt } from "components/CoverArt";
import { Footer } from "components/Footer";
import { Game } from "components/Game";
import { GameForm } from "components/GameForm";
import { ShortcutsHelpModal } from "components/ShortcutsHelp";
import React from "react";
import styles from "./App.module.css";

interface IState {
    playerNames: string[];
    hasConfirmed: boolean;
    cpuPlayerCount: number;
    gameNumber: number;
    showShortcutsModal: boolean;
}

export const App: React.FC = () => {
    // TODO: useReducer
    const [state, setState] = React.useState<IState>({
        playerNames: [""],
        hasConfirmed: false,
        cpuPlayerCount: 1,
        gameNumber: 1,
        showShortcutsModal: false,
    });

    const onChange = (text: string, idx: number) => {
        setState((prevState) => ({
            ...prevState,
            playerNames: prevState.playerNames.map((name, i) => i === idx ? text : name),
        }));
    }

    return (
        <>
            <main>
                <Container>
                    <CoverArt />
                    <h1 className={ styles.landingTitle }>nile</h1>
                    <h2 className={ styles.subtitle }>a path-creating game</h2>
                    { state.hasConfirmed
                        // Incrementing key will remount Game
                        ? <>
                            {/* TODO: Color green */}
                            <Button title="New game"
                                // TODO: confirm starting new game
                                onClick={ () => setState((prevState) => ({...prevState, hasConfirmed: false, gameNumber: prevState.gameNumber + 1})) }
                            >
                                New game
                            </Button>
                            <Button title="Shortcuts help"
                                onClick={ () => setState((prevState) => ({...prevState, showShortcutsModal: true}))}
                            >
                                Shortcuts Help
                            </Button>
                            { state.showShortcutsModal &&
                                <ShortcutsHelpModal
                                    dismiss={ () => setState((prevState) => ({...prevState, showShortcutsModal: false}))}
                                /> }
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
            </main>
            <footer>
                <Footer />
            </footer>
        </>
    );
};
App.displayName = "App";
