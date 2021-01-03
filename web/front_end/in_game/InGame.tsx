import { Button } from "components/Button";
import colors from "components/colors.module.css";
import { Container } from "components/Container";
import { Action, IState } from "lib/app_state";
import React from "react";
import { Game } from "./Game";
import { Header } from "./Header";
import { ShortcutsHelpModal } from "./ShortcutsHelp";

interface IProps {
    state: IState;
    dispatch: React.Dispatch<Action>;
}

export const InGame: React.FC<IProps> = ({state, dispatch}) => {
    return (
        <Container>
            <Header />
            <Button title="New game"
                className={ colors.riverTurquoiseBg }
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
        </Container>
    );
}
InGame.displayName = "InGame";
