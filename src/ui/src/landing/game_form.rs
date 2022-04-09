use yew::prelude::*;

use crate::{
    app::PlayerNameChange,
    components::{
        carbon_icon::{CarbonIcon, Size},
        Button,
    },
};

pub struct GameForm {}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub player_names: Vec<String>,
    pub cpu_player_count: u8,
    pub dispatch: Callback<crate::app::Msg>,
}

impl Component for GameForm {
    type Properties = Props;
    type Message = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let total_player_count =
            ctx.props().player_names.len() + ctx.props().cpu_player_count as usize;
        let can_start = (2..=4).contains(&total_player_count);
        let on_add_player = ctx
            .props()
            .dispatch
            .reform(move |_| crate::app::Msg::AddPlayer);
        let on_rm_player = ctx
            .props()
            .dispatch
            .reform(move |_| crate::app::Msg::RemovePlayer);
        let on_add_cpu_player = ctx
            .props()
            .dispatch
            .reform(move |_| crate::app::Msg::AddCpuPlayer);
        let on_rm_cpu_player = ctx
            .props()
            .dispatch
            .reform(move |_| crate::app::Msg::RemoveCpuPlayer);
        let on_start = ctx
            .props()
            .dispatch
            .reform(move |_| crate::app::Msg::Confirm);
        let on_reset = ctx.props().dispatch.reform(move |_| crate::app::Msg::Reset);
        html! {
            <form class="game-form">
                { for { ctx.props().player_names
                    .iter()
                    .enumerate()
                    .map(|(i, name)| self.view_player_name_input(ctx, i, name))
                } }
                <Button title="Add player"
                    aria_label="Add player"
                    class={ classes!("nile-blue-bg") }
                    is_enabled={ total_player_count < 4 }
                    on_click={ on_add_player }
                >
                    <CarbonIcon name="add_filled" size={ Size::S16 } />
                </Button>
                <Button title="Remove player"
                    aria_label="Remove player"
                    class={ classes!("nile-blue-bg") }
                    is_enabled={ ctx.props().player_names.len() > 1 }
                    on_click={ on_rm_player }
                >
                    <CarbonIcon name="subtract" size={ Size::S16 } />
                </Button>
                <br />
                <span class="cpu-count">{ format!("CPU players: {}", ctx.props().cpu_player_count) }</span>
                <Button title="Add CPU player"
                    aria_label="Add CPU player"
                    class={ classes!("nile-blue-bg") }
                    is_enabled={ total_player_count < 4 }
                    on_click={ on_add_cpu_player }
                >
                    <CarbonIcon name="add_filled" size={ Size::S16 } />
                </Button>
                <Button title="Remove CPU player"
                    aria_label="Remove CPU player"
                    class={ classes!("nile-blue-bg") }
                    is_enabled={ total_player_count > 1 && ctx.props().cpu_player_count > 0 }
                    on_click={ on_rm_cpu_player }
                >
                    <CarbonIcon name="subtract" size={ Size::S16 } />
                </Button>
                <br />
                <Button title={ if can_start { "Start new game" } else { "Need at least two players" } }
                    class={ classes!("river-turquoise-bg") }
                    aria_label="Start new game"
                    is_enabled={ can_start }
                    on_click={ on_start }
                >
                    { "Start" }
                </Button>
                <Button title={ "Reset" }
                    class={ classes!("red-bg") }
                    aria_label="Reset game form"
                    on_click={ on_reset }
                >
                    { "Reset" }
                </Button>
            </form>
        }
    }
}

impl GameForm {
    fn view_player_name_input(&self, ctx: &Context<Self>, i: usize, name: &str) -> Html {
        let i_str = i.to_string();
        let on_change = {
            let dispatch = ctx.props().dispatch.clone();
            Callback::from(move |e: Event| {
                if let Some(name) = e.value_of().as_string() {
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
