use super::{board::Board, utils::update_if_changed};
use crate::{
    components::{controls::Controls, modal::error::ErrorModal, player::Players},
    state::{Action, GameStore, NewGameOptions, Rotation, SelectRackTile},
};

use nile::console;
use yew::{
    prelude::*,
    services::{keyboard::KeyListenerHandle, KeyboardService},
    utils::document,
};
use yewdux::{
    component::WithDispatch,
    prelude::{DispatchProps, DispatchPropsMut, Dispatcher},
};

pub struct GameImpl {
    props: Props,
    _handle: KeyListenerHandle,
}
pub type Game = WithDispatch<GameImpl>;

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub dispatch: DispatchProps<GameStore>,
    pub player_names: Vec<String>,
    pub cpu_player_count: u8,
}

impl DispatchPropsMut for Props {
    type Store = GameStore;

    fn dispatch(&mut self) -> &mut DispatchProps<Self::Store> {
        &mut self.dispatch
    }
}

impl Component for GameImpl {
    type Properties = Props;
    type Message = ();

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        let dispatch = &props.dispatch;
        dispatch.send(Action::NewGame(NewGameOptions {
            player_names: props.player_names.clone(),
            cpu_player_count: props.cpu_player_count,
        }));
        let handle = {
            let rotate_selected = dispatch.callback(Action::RotateSelectedTile);
            let remove_selected = dispatch.callback(|_| Action::RemoveSelectedTile);
            let undo = dispatch.callback(|_| Action::Undo);
            let redo = dispatch.callback(|_| Action::Redo);
            let end_turn = dispatch.callback(|_| Action::EndTurn);
            let cant_play = dispatch.callback(|_| Action::CantPlay);
            let select_rack_tile =
                dispatch.callback(|rack_idx| Action::SelectRackTile(SelectRackTile { rack_idx }));
            let dismiss = dispatch.callback(|_| Action::Dismiss);
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
                                .next()
                                .and_then(|c| c.to_digit(10))
                            {
                                let idx = (num - 1) as u8;
                                select_rack_tile.emit(idx);
                            }
                        }
                        "Escape" => dismiss.emit(()),
                        _ => (),
                    }
                }),
            )
        };
        Self {
            props,
            _handle: handle,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        update_if_changed(&mut self.props, props)
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

impl GameImpl {
    fn view_error_modal(&self) -> Html {
        let dispatch = &self.props.dispatch;
        dispatch
            .state()
            .modal
            .as_ref()
            .map(|modal| match modal {
                crate::state::Modal::EndOfGame(msg) => msg,
                crate::state::Modal::Error(msg) => msg,
            })
            .map_or(html! {}, |msg| {
                let dismiss = dispatch.callback(|_| Action::Dismiss);
                html! {
                    <ErrorModal msg={ msg.clone() }
                        dismiss={ dismiss }
                    />
                }
            })
    }
}
