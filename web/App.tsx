import { Button } from "components/Button";
import { Container } from "components/Container";
import { Footer } from "components/Footer";
import { Game } from "components/Game";
import { GameForm } from "components/GameForm";
import { Header } from "components/Header";
import { ShortcutsHelpModal } from "components/ShortcutsHelp";
import { initState, reducer } from "lib/app_state";
import React from "react";

export const App: React.FC = () => {
    const [state, dispatch] = React.useReducer(reducer, [], initState);

    return (
        <>
            <main>
                <Container>
                    <Header />
                    { state.hasConfirmed
                        // Incrementing key will remount Game
                        ? <>
                            {/* TODO: Color green */}
                            <Button title="New game"
                                // TODO: confirm starting new game
                                onClick={ () => dispatch({type: "newGame"}) }
                            >
                                New game
                            </Button>
                            <Button title="Shortcuts help"
                                onClick={ () => dispatch({type: "setShowShortcutsModal", showShortcutsModal: true}) }
                            >
                                Shortcuts Help
                            </Button>
                            { state.showShortcutsModal &&
                                <ShortcutsHelpModal
                                    dismiss={ () => dispatch({type: "setShowShortcutsModal", showShortcutsModal: false}) }
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
                                onPlayerNameChange={ (name, idx) => dispatch({type: "updatePlayer", name, idx}) }
                                onAddPlayer={ () => dispatch({type: "addPlayer"}) }
                                // Remove last player
                                onRemovePlayer={ () => dispatch({type: "removePlayer"}) }
                                onAddCpuPlayer={ () => dispatch({type: "addCpuPlayer"}) }
                                onRemoveCpuPlayer={ () => dispatch({type: "removeCpuPlayer"}) }
                                onClickStart={ () => dispatch({type: "startGame"}) }
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
