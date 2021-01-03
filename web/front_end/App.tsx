import { Container } from "components/Container";
import { Footer } from "components/Footer";
import { InGame } from "front_end/in_game/InGame";
import { Landing } from "front_end/landing/Landing";
import { initState, reducer } from "lib/app_state";
import React from "react";

export const App: React.FC = () => {
    const [state, dispatch] = React.useReducer(reducer, [], initState);

    return (
        <>
            <main>
                { state.hasConfirmed
                    ? <InGame state={ state } dispatch={ dispatch } />
                    : <Landing state={ state } dispatch={ dispatch } />}
            </main>
            <footer>
                <Footer />
            </footer>
        </>
    );
};
App.displayName = "App";
