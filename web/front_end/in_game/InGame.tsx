import { Button } from "components/Button";
import { Action, IState } from "lib/app_state";
import React from "react";
import { Game } from "./Game";
import { ShortcutsHelpModal } from "./ShortcutsHelp";

interface IProps {
    state: IState;
    dispatch: React.Dispatch<Action>;
}

export const InGame: React.FC<IProps> = ({state, dispatch}) => {
    return (
        <>
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
    );
}
InGame.displayName = "InGame";
