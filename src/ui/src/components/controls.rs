use yew::prelude::*;
use yewdux::{
    component::WithDispatch,
    prelude::{DispatchProps, Dispatcher},
};

use crate::{
    components::{
        button::Button,
        carbon_icon::{CarbonIcon, Size},
    },
    state::{Action, GameStore, Rotation},
};

use super::utils::update_if_changed;

pub struct ControlsImpl {
    props: DispatchProps<GameStore>,
}
pub type Controls = WithDispatch<ControlsImpl>;

impl Component for ControlsImpl {
    type Properties = DispatchProps<GameStore>;
    type Message = ();

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        // TODO: narrow to just used props
        update_if_changed(&mut self.props, props)
    }

    fn view(&self) -> Html {
        let state = self.props.state();
        let on_rotate_counterclockwise = self
            .props
            .callback_once(|_| Action::RotateSelectedTile(Rotation::Counterclockwise));
        let on_rotate_clockwise = self
            .props
            .callback_once(|_| Action::RotateSelectedTile(Rotation::Clockwise));
        let on_remove_tile = self.props.callback(|_| Action::RemoveTile);
        let on_undo = self.props.callback(|_| Action::Undo);
        let on_redo = self.props.callback(|_| Action::Redo);
        let on_end_turn = self.props.callback(|_| Action::EndTurn);
        let on_cant_play = self.props.callback(|_| Action::CantPlay);
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
                <Button is_enabled={ !state.current_turn_tiles.is_empty() }
                    on_click={ on_remove_tile }
                    title="Remove tile"
                    aria_label="Remove selected tile from the board"
                >
                    <CarbonIcon name="trash_can" size={ Size::S24 } />
                </Button>

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
                <Button is_enabled={ !state.current_turn_tiles.is_empty() }
                    on_click={ on_end_turn }
                    title="End turn"
                    aria_label="End turn"
                >
                    <CarbonIcon name="checkmark" size={ Size::S24 } />
                    { "End Turn" }
                </Button>
                <Button is_enabled={ state.current_turn_tiles.is_empty() }
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
