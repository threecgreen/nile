use yew::prelude::*;

use crate::{
    app::PlayerNameChange,
    components::{
        carbon_icon::{CarbonIcon, Size},
        Button,
    },
};

use super::utils::update_if_changed;

pub struct GameForm {
    props: Props,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub player_names: Vec<String>,
    pub cpu_player_count: u8,
    pub dispatch: Callback<crate::app::Msg>,
}

impl Component for GameForm {
    type Properties = Props;
    type Message = ();

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        update_if_changed(&mut self.props, props)
    }

    fn view(&self) -> Html {
        let total_player_count =
            self.props.player_names.len() + self.props.cpu_player_count as usize;
        let can_start = total_player_count >= 2 && total_player_count <= 4;
        let on_add_player = {
            let dispath = self.props.dispatch.clone();
            Callback::from(move |_| dispath.emit(crate::app::Msg::AddPlayer))
        };
        let on_rm_player = {
            let dispath = self.props.dispatch.clone();
            Callback::from(move |_| dispath.emit(crate::app::Msg::RemovePlayer))
        };
        let on_add_cpu_player = {
            let dispath = self.props.dispatch.clone();
            Callback::from(move |_| dispath.emit(crate::app::Msg::AddCpuPlayer))
        };
        let on_rm_cpu_player = {
            let dispath = self.props.dispatch.clone();
            Callback::from(move |_| dispath.emit(crate::app::Msg::RemoveCpuPlayer))
        };
        let on_start = {
            let dispatch = self.props.dispatch.clone();
            Callback::from(move |_| dispatch.emit(crate::app::Msg::Confirm))
        };
        html! {
            <form class="game-form">
                { for { self.props.player_names
                    .iter()
                    .enumerate()
                    .map(|(i, name)| self.view_player_name_input(i, name))
                } }
                <Button title="Add player"
                    aria_label="Add player"
                    is_enabled={ total_player_count < 4 }
                    on_click={ on_add_player }
                >
                    <CarbonIcon name="add_filled" size={ Size::S16 } />
                </Button>
                <Button title="Remove player"
                    aria_label="Remove player"
                    is_enabled={ self.props.player_names.len() > 1 }
                    on_click={ on_rm_player }
                >
                    <CarbonIcon name="subtract" size={ Size::S16 } />
                </Button>
                <br />
                <span class="cpu-count">{ format!("CPU players: {}", self.props.cpu_player_count) }</span>
                <Button title="Add CPU player"
                    aria_label="Add CPU player"
                    is_enabled={ total_player_count < 4 }
                    on_click={ on_add_cpu_player }
                >
                    <CarbonIcon name="add_filled" size={ Size::S16 } />
                </Button>
                <Button title="Remove CPU player"
                    aria_label="Remove CPU player"
                    is_enabled={ total_player_count > 1 && self.props.cpu_player_count > 0 }
                    on_click={ on_rm_cpu_player }
                >
                    <CarbonIcon name="subtract" size={ Size::S16 } />
                </Button>
                <br />
                <Button title={ if can_start { "Start new game" } else { "Need at least two players" } }
                    aria_label="Start new game"
                    is_enabled={ can_start }
                    on_click={ on_start }
                >
                    { "Start" }
                </Button>
            </form>
        }
    }
}

impl GameForm {
    fn view_player_name_input(&self, i: usize, name: &str) -> Html {
        let i_str = i.to_string();
        let on_change = {
            let dispatch = self.props.dispatch.clone();
            Callback::from(move |e: ChangeData| {
                if let ChangeData::Value(name) = e {
                    dispatch.emit(crate::app::Msg::PlayerNameChange(PlayerNameChange {
                        idx: i,
                        name,
                    }))
                }
            })
        };
        html! {
            <>
                <input id={ i_str.clone() }
                    value={ name.to_owned() }
                    onchange={ on_change }
                    required={ true }
                    placeholder="Name"
                />
                <label for={ i_str }>
                    { format!("Player {}", i + 1) }
                </label>
            </>
        }
    }
}
