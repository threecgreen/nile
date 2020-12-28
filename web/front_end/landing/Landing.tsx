import React from "react";
import { IState, Action } from "lib/app_state";
import { GameForm } from "front_end/landing/GameForm";

interface IProps {
    state: IState;
    dispatch: React.Dispatch<Action>;
}

export const Landing: React.FC<IProps> = ({state, dispatch}) => {
    return (
        <>
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
        </>
    );
}
Landing.displayName = "Landing";
