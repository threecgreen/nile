import { GameForm } from "front_end/landing/GameForm";
import { Action, IState } from "lib/app_state";
import React from "react";
import { ClickButton, LinkButton } from "./Buttons";
import styles from "./Landing.module.css";

interface IProps {
    state: IState;
    dispatch: React.Dispatch<Action>;
}

export const Landing: React.FC<IProps> = ({state, dispatch}) => {
    return (
        // FIXME: move Container here and make narrower
        <>
            <div className={ styles.centerContent }>
                <LinkButton href="#about">
                    about
                </LinkButton>
                <LinkButton href="#how-to-play">
                    how to play
                </LinkButton>
                <ClickButton onClick={ () => dispatch({type: "setShowNewGameForm", showNewGameForm: true}) }>
                    new game
                </ClickButton>
            </div>
            { state.showNewGameForm &&
                <section>
                    <h3 className={ styles.sectionTitle }>New game options</h3>
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
                </section>
            }
            <section>
                <h3 className={ styles.sectionTitle }><a id="about">about</a></h3>
                <p>
                    A web version of a 1960s tile-based board game, in nile players take turns extending the course of the river, getting bonuses, and setting up opponents for penalties.
                </p>
                <p>
                    Play against other people, the AI, or a mix. Supports 2–4 players.
                </p>
            </section>

            <h3 className={ styles.sectionTitle }><a id="how-to-play">how to play</a></h3>
            <section>

            </section>
        </>
    );
}
Landing.displayName = "Landing";
