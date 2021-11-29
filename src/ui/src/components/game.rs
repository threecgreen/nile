use std::rc::Rc;

use super::board::Board;
use crate::{
    components::{controls::Controls, modal::error::ErrorModal, player::Players},
    state::{Action, GameStore, Rotation, SelectRackTile, State},
};

use nile::console;
use yew::{
    prelude::*,
    services::{keyboard::KeyListenerHandle, KeyboardService},
    utils::document,
};
use yewdux::prelude::{Dispatch, Dispatcher};

pub struct Game {
    dispatch: Dispatch<GameStore>,
    state: Rc<State>,
    link: ComponentLink<Self>,
    handle: KeyListenerHandle,
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
        let handle = {
            let rotate_selected = dispatch.callback(|r| Action::RotateSelectedTile(r));
            let remove_selected = dispatch.callback(|_| Action::RemoveSelectedTile);
            let undo = dispatch.callback(|_| Action::Undo);
            let redo = dispatch.callback(|_| Action::Redo);
            let end_turn = dispatch.callback(|_| Action::EndTurn);
            let cant_play = dispatch.callback(|_| Action::CantPlay);
            let select_rack_tile =
                dispatch.callback(|rack_idx| Action::SelectRackTile(SelectRackTile { rack_idx }));
            KeyboardService::register_key_down(
                &document(),
                Callback::from(move |keyboard_event: KeyboardEvent| {
                    console::debug("Keyboard callback called");
                    if keyboard_event.ctrl_key()
                        || keyboard_event.alt_key()
                        || keyboard_event.meta_key()
                    {
                        return;
                    }
                    match keyboard_event.key().as_str() {
                        "q" => rotate_selected.emit(Rotation::Counterclockwise),
                        "e" => rotate_selected.emit(Rotation::Clockwise),
                        "x" => remove_selected.emit(()),
                        "u" => undo.emit(()),
                        "r" => redo.emit(()),
                        "E" => end_turn.emit(()),
                        "C" => cant_play.emit(()),
                        "1" | "2" | "3" | "4" | "5" => {
                            if let Some(num) = keyboard_event
                                .key()
                                .as_str()
                                .chars()
                                .nth(0)
                                .and_then(|c| c.to_digit(10))
                            {
                                let idx = (num - 1) as u8;
                                select_rack_tile.emit(idx);
                            }
                        }
                        _ => (),
                    }
                }),
            )
        };
        Self {
            dispatch,
            state: Rc::new(State::new_game(props.player_names, props.cpu_player_count)),
            link,
            handle,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::State(state) => self.state = state,
        };
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.state = Rc::new(State::new_game(props.player_names, props.cpu_player_count));
        true
    }

    fn view(&self) -> Html {
        html! {
            <>
                <section>
                    <Controls />
                    <Board />
                    { self.view_error_modal() }
                </section>
                <section>
                    <Players />
                </section>
            </>
        }
    }
}

impl Game {
    fn view_error_modal(&self) -> Html {
        self.state
            .modal
            .as_ref()
            .map(|modal| match modal {
                crate::state::Modal::EndOfGame(msg) => msg,
                crate::state::Modal::Error(msg) => msg,
            })
            .map_or(html! {}, |msg| {
                let dismiss = self.dispatch.callback(|_| Action::Dismiss);
                html! {
                    <ErrorModal msg={ msg.clone() }
                        dismiss={ dismiss }
                    />
                }
            })
    }
}
