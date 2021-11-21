use std::rc::Rc;

use super::board::Board;
use crate::{
    components::{
        carbon_icon::{CarbonIcon, Size},
        controls::Controls,
    },
    state::{GameStore, State},
};

use yew::prelude::*;
use yewdux::prelude::Dispatch;

pub struct Game {
    dispatch: Dispatch<GameStore>,
    state: Rc<State>,
    link: ComponentLink<Self>,
}

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    pub player_names: Vec<String>,
    pub cpu_player_count: u8,
}

pub enum Msg {
    State(Rc<State>),
}

impl Component for Game {
    type Properties = Props;
    type Message = Msg;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let dispatch = Dispatch::bridge_state(link.callback(Msg::State));
        Self {
            dispatch,
            state: Rc::new(State::new_game(props.player_names, props.cpu_player_count)),
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::State(state) => self.state = state,
        };
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        todo!("Implement new game")
    }

    fn view(&self) -> Html {
        html! {
            <>
                <section>
                    <Controls />
                    <Board />
                </section>
                <section>
                </section>
            </>
        }
    }
}
