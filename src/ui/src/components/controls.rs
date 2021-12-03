use nile::{Tile, TilePath, TILE_PATHS};
use yew::prelude::*;
use yewdux::{
    component::WithDispatch,
    prelude::{DispatchProps, Dispatcher},
};

use crate::{
    components::{
        button::Button,
        carbon_icon::{CarbonIcon, Size},
        tile::rack_tile::RackTile,
    },
    state::{Action, GameStore, Rotation},
};

use super::utils::update_if_changed;

pub struct ControlsImpl {
    props: DispatchProps<GameStore>,
    link: ComponentLink<Self>,
    is_tile_path_selector_open: bool,
}
pub type Controls = WithDispatch<ControlsImpl>;
pub enum Msg {
    SetIsTilePathSelectorOpen(bool),
}

impl Component for ControlsImpl {
    type Properties = DispatchProps<GameStore>;
    type Message = Msg;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
            is_tile_path_selector_open: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SetIsTilePathSelectorOpen(is_tile_path_selector_open) => update_if_changed(
                &mut self.is_tile_path_selector_open,
                is_tile_path_selector_open,
            ),
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        // TODO: narrow to just used props

        if update_if_changed(&mut self.props, props) {
            if !self.props.state().selected_is_universal() {
                self.is_tile_path_selector_open = false;
            }
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let state = self.props.state();
        let selected_is_universal = state.selected_is_universal();
        let on_rotate_counterclockwise = self
            .props
            .callback(|_| Action::RotateSelectedTile(Rotation::Counterclockwise));
        let on_rotate_clockwise = self
            .props
            .callback(|_| Action::RotateSelectedTile(Rotation::Clockwise));
        let on_remove_tile = self.props.callback(|_| Action::RemoveSelectedTile);
        let on_undo = self.props.callback(|_| Action::Undo);
        let on_redo = self.props.callback(|_| Action::Redo);
        let on_end_turn = self.props.callback(|_| Action::EndTurn);
        let on_cant_play = self.props.callback(|_| Action::CantPlay);
        let on_click_dropdown = {
            let is_tile_path_selector_open = self.is_tile_path_selector_open;
            self.link
                .callback(move |_| Msg::SetIsTilePathSelectorOpen(!is_tile_path_selector_open))
        };
        html! {
            <div class="controls">
                <Button is_enabled={ state.has_selected_board_tile() }
                    on_click={ on_rotate_counterclockwise }
                    title="Rotate tile counter-clockwise"
                    aria_label="Rotate selected tile counter-clockwise"
                >
                    <CarbonIcon name="rotate_counterclockwise" size={ Size::S24 } />
                </Button>
                <Button is_enabled={ state.has_selected_board_tile() }
                    on_click={ on_rotate_clockwise }
                    title="Rotate tile clockwise"
                    aria_label="Rotate selected tile clockwise"
                >
                    <CarbonIcon name="rotate_clockwise" size={ Size::S24 } />
                </Button>
                <Button is_enabled={ !state.nile.current_turn_placements().is_empty() }
                    on_click={ on_remove_tile }
                    title="Remove tile"
                    aria_label="Remove selected tile from the board"
                >
                    <CarbonIcon name="trash_can" size={ Size::S24 } />
                </Button>
                <div class={ if selected_is_universal { "dropdown" } else { "dropdown disabled" } }>
                    <Button aria_label="Select tile path for universal tile"
                        class="dropdown"
                        is_enabled={ selected_is_universal }
                        on_click={ on_click_dropdown }
                    >
                        { "Tile Path " }<CarbonIcon name="down_to_buttom" size={ Size::S24 } />
                    </Button>
                    { self.view_dropdown() }
                </div>
                <Button is_enabled={ state.can_undo() }
                    on_click={ on_undo }
                    title="Undo"
                    aria_label="Undo the last move"
                >
                    <CarbonIcon name="undo" size={ Size::S24 } />
                </Button>
                <Button is_enabled={ state.can_redo() }
                    on_click={ on_redo }
                    title="Redo"
                    aria_label="Redo an undone move"
                >
                    <CarbonIcon name="redo" size={ Size::S24 } />
                </Button>
                <Button is_enabled={ !state.nile.current_turn_placements().is_empty() }
                    on_click={ on_end_turn }
                    title="End turn"
                    aria_label="End turn"
                >
                    <CarbonIcon name="checkmark" size={ Size::S24 } />
                    { "End Turn" }
                </Button>
                <Button is_enabled={ state.nile.current_turn_placements().is_empty() }
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

impl ControlsImpl {
    fn view_dropdown(&self) -> Html {
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
        let on_click = self.props.callback(move |e: MouseEvent| {
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
