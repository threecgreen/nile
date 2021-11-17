use crate::components::Footer;
use yew::prelude::*;

pub struct App {
    link: ComponentLink<App>,
    player_names: Vec<String>,
    has_confirmed: bool,
    cpu_player_count: u8,
    game_number: u32,
    show_shortcuts_modal: bool,
}

pub enum Msg {}

impl Component for App {
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
                <footer>
                    <Footer />
                </footer>
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
