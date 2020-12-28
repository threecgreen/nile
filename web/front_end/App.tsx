import { Container } from "components/Container";
import { Footer } from "components/Footer";
import { Header } from "components/Header";
import { InGame } from "front_end/in_game/InGame";
import { Landing } from "front_end/landing/Landing";
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
                        ? <InGame state={ state } dispatch={ dispatch } />
                        : <Landing state={ state } dispatch={ dispatch } />}
                </Container>
            </main>
            <footer>
                <Footer />
            </footer>
        </>
    );
};
App.displayName = "App";
