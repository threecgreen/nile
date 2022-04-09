use std::rc::Rc;

use super::{
    board::Board,
    controls::Controls,
    player::Players,
    state::{Action, Dispatch, NewGameOptions, Rotation, SelectRackTile, State, UpdateStateMsg},
};
use crate::components::ErrorModal;

use yew::prelude::*;

pub struct Game {
    dispatch: Dispatch,
    state: Rc<State>,
}

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    pub player_names: Vec<String>,
    pub cpu_player_count: u8,
}

impl Component for Game {
    type Properties = Props;
    type Message = UpdateStateMsg;

    fn create(ctx: &Context<Self>) -> Self {
        let callback = ctx.link().callback(UpdateStateMsg);
        let dispatch = Dispatch::subscribe(callback);
        dispatch.apply(Action::NewGame(NewGameOptions {
            player_names: ctx.props().player_names.clone(),
            cpu_player_count: ctx.props().cpu_player_count,
        }));
        let state = dispatch.get();
        Self { dispatch, state }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let dispatch = &self.dispatch;
        let rotate_selected = dispatch.apply_callback(Action::RotateSelectedTile);
        let remove_selected = dispatch.apply_callback(|_| Action::RemoveSelectedTile);
        let undo = dispatch.apply_callback(|_| Action::Undo);
        let redo = dispatch.apply_callback(|_| Action::Redo);
        let end_turn = dispatch.apply_callback(|_| Action::EndTurn);
        let cant_play = dispatch.apply_callback(|_| Action::CantPlay);
        let select_rack_tile =
            dispatch.apply_callback(|rack_idx| Action::SelectRackTile(SelectRackTile { rack_idx }));
        let dismiss = dispatch.apply_callback(|_| Action::Dismiss);
        let on_key_down = Callback::from(move |keyboard_event: KeyboardEvent| {
            if keyboard_event.ctrl_key() || keyboard_event.alt_key() || keyboard_event.meta_key() {
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
        });
        // TODO: fix global/window-level key listening
        html! {
            <div onkeydown={ on_key_down }>
                <section>
                    <Controls />
                    <Board />
                    { self.view_error_modal() }
                </section>
                <section>
                    <Players />
                </section>
            </div>
        }
    }
}

impl Game {
    fn view_error_modal(&self) -> Html {
        self.state
            .modal
            .as_ref()
            .map(|modal| match modal {
                super::state::Modal::EndOfGame(msg) => msg,
                super::state::Modal::Error(msg) => msg,
            })
            .map_or(html! {}, |msg| {
                let dismiss = self.dispatch.apply_callback(|_| Action::Dismiss);
                html! {
                    <ErrorModal msg={ msg.clone() }
                        dismiss={ dismiss }
                    />
                }
            })
    }
}
