use std::sync::mpsc::channel;

use yew::prelude::*;

pub struct Model {
    player_names: Vec<String>,
    has_confirmed: bool,
    cpu_player_count: u8,
    game_number: u32,
    show_shortcuts_modal: bool,

    link: ComponentLink<Model>,
}

pub enum Msg {}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            player_names: Vec::default(),
            has_confirmed: false,
            cpu_player_count: 1,
            game_number: 1,
            show_shortcuts_modal: false,
            link,
        }
    }

    fn view(&self) -> Html {
        html! {
            <>
                <main>
                    <h1>{ "Nile" }</h1>
                </main>
            </>
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {}
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }
}
