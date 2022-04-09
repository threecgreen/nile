use std::rc::Rc;

use nile::{Tile, TilePath, TILE_PATHS};
use yew::prelude::*;

use super::state::{Action, Dispatch, Rotation, State};
use crate::components::{
    carbon_icon::{CarbonIcon, Size},
    utils::update_if_changed,
    Button, RackTile,
};

pub struct Controls {
    dispatch: Dispatch,
    state: Rc<State>,
    is_tile_path_selector_open: bool,
}
pub enum Msg {
    SetIsTilePathSelectorOpen(bool),
    UpdateState(Rc<State>),
}

impl Component for Controls {
    type Properties = ();
    type Message = Msg;

    fn create(ctx: &Context<Self>) -> Self {
        let callback = ctx.link().callback(Msg::UpdateState);
        let dispatch = Dispatch::subscribe(callback);
        let state = dispatch.get();
        Self {
            dispatch,
            state,
            is_tile_path_selector_open: false,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetIsTilePathSelectorOpen(is_tile_path_selector_open) => update_if_changed(
                &mut self.is_tile_path_selector_open,
                is_tile_path_selector_open,
            ),
            Msg::UpdateState(new_state) => {
                if !new_state.selected_is_universal() {
                    self.is_tile_path_selector_open = false;
                }
                // TODO: narrow to just used props
                self.state = new_state;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let state = &self.state;
        let selected_is_universal = state.selected_is_universal();
        let on_rotate_counterclockwise = self
            .dispatch
            .apply_callback(|_| Action::RotateSelectedTile(Rotation::Counterclockwise));
        let on_rotate_clockwise = self
            .dispatch
            .apply_callback(|_| Action::RotateSelectedTile(Rotation::Clockwise));
        let on_remove_tile = self.dispatch.apply_callback(|_| Action::RemoveSelectedTile);
        let on_undo = self.dispatch.apply_callback(|_| Action::Undo);
        let on_redo = self.dispatch.apply_callback(|_| Action::Redo);
        let on_end_turn = self.dispatch.apply_callback(|_| Action::EndTurn);
        let on_cant_play = self.dispatch.apply_callback(|_| Action::CantPlay);
        let on_click_dropdown = {
            let is_tile_path_selector_open = self.is_tile_path_selector_open;
            ctx.link()
                .callback(move |_| Msg::SetIsTilePathSelectorOpen(!is_tile_path_selector_open))
        };
        html! {
            <div class="controls">
                <Button is_enabled={ state.has_selected_board_tile() }
                    class={ classes!("nile-blue-bg") }
                    on_click={ on_rotate_counterclockwise }
                    title="Rotate tile counter-clockwise"
                    aria_label="Rotate selected tile counter-clockwise"
                >
                    <CarbonIcon name="rotate_counterclockwise" size={ Size::S24 } />
                </Button>
                <Button is_enabled={ state.has_selected_board_tile() }
                    class={ classes!("nile-blue-bg") }
                    on_click={ on_rotate_clockwise }
                    title="Rotate tile clockwise"
                    aria_label="Rotate selected tile clockwise"
                >
                    <CarbonIcon name="rotate_clockwise" size={ Size::S24 } />
                </Button>
                <Button is_enabled={ !state.nile.current_turn_placements().is_empty() }
                    class={ classes!("red-bg") }
                    on_click={ on_remove_tile }
                    title="Remove tile"
                    aria_label="Remove selected tile from the board"
                >
                    <CarbonIcon name="trash_can" size={ Size::S24 } />
                </Button>
                <div class={ classes!("dropdown", (!selected_is_universal).then(|| "disabled")) }>
                    <Button aria_label="Select tile path for universal tile"
                        class={ classes!("dropdown", "nile-blue-bg") }
                        is_enabled={ selected_is_universal }
                        on_click={ on_click_dropdown }
                    >
                        <CarbonIcon name="down_to_buttom" size={ Size::S24 } />
                        { "Tile Path " }
                    </Button>
                    { self.view_dropdown(ctx) }
                </div>
                <Button is_enabled={ state.can_undo() }
                    class={ classes!("nile-blue-bg") }
                    on_click={ on_undo }
                    title="Undo"
                    aria_label="Undo the last move"
                >
                    <CarbonIcon name="undo" size={ Size::S24 } />
                </Button>
                <Button is_enabled={ state.can_redo() }
                    class={ classes!("nile-blue-bg") }
                    on_click={ on_redo }
                    title="Redo"
                    aria_label="Redo an undone move"
                >
                    <CarbonIcon name="redo" size={ Size::S24 } />
                </Button>
                <Button is_enabled={ !state.nile.current_turn_placements().is_empty() }
                    class={ classes!("river-turquoise-bg") }
                    on_click={ on_end_turn }
                    title="End turn"
                    aria_label="End turn"
                >
                    <CarbonIcon name="checkmark" size={ Size::S24 } />
                    { "End Turn" }
                </Button>
                <Button is_enabled={ state.nile.current_turn_placements().is_empty() }
                    class={ classes!("red-bg") }
                    on_click={ on_cant_play }
                    title="Can’t play"
                    aria_label="Can’t play"
                >
                    <CarbonIcon name="close" size={ Size::S24 } />
                    { "Can’t play" }
                </Button>
            </div>
        }
    }
}

impl Controls {
    fn view_dropdown(&self, _ctx: &Context<Self>) -> Html {
        if self.is_tile_path_selector_open {
            html! {
                <div class="dropdown-content">
                    { for TILE_PATHS.map(|tp| self.view_tile_path_selection(tp)) }
                </div>
            }
        } else {
            html! {}
        }
    }

    fn view_tile_path_selection(&self, tile_path: TilePath) -> Html {
        let on_click = self.dispatch.apply_callback(move |e: MouseEvent| {
            e.prevent_default();
            Action::UpdateSelectedUniversalPath(tile_path)
        });
        html! {
            <a onclick={ on_click }>
                <RackTile tile={ Tile::from(tile_path) } is_selected={ false } />
            </a>
        }
    }
}
