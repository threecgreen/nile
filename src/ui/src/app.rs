use crate::components::{Container, Footer, Game};
use yew::prelude::*;

pub struct App {
    link: ComponentLink<App>,
    player_names: Vec<String>,
    has_confirmed: bool,
    cpu_player_count: u8,
    game_number: u32,
    show_shortcuts_modal: bool,
}

pub enum Msg {
    PlayerNameChange(PlayerNameChange),
    RemovePlayer,
    AddCpuPlayer,
    RemoveCpuPlayer,
    Confirm,
}

pub struct PlayerNameChange {
    idx: usize,
    name: String,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            // player_names: Vec::default(),
            player_names: vec!["Player1".to_owned()],
            has_confirmed: false,
            cpu_player_count: 1,
            game_number: 1,
            show_shortcuts_modal: false,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::PlayerNameChange(PlayerNameChange { idx, name }) => {
                if let Some(player_name) = self.player_names.get_mut(idx) {
                    *player_name = name;
                    true
                } else {
                    false
                }
            }
            Msg::RemovePlayer => self.player_names.pop().is_some(),
            Msg::AddCpuPlayer => {
                self.cpu_player_count += 1;
                true
            }
            Msg::RemoveCpuPlayer => {
                self.cpu_player_count -= 0;
                true
            }
            Msg::Confirm => {
                self.has_confirmed = true;
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <main>
                    <Container>
                        <h1>{ "Nile" }</h1>
                        <Game player_names={ self.player_names.clone() }
                            cpu_player_count={ self.cpu_player_count }
                        />
                    </Container>
                </main>
                <footer>
                    <Footer />
                </footer>
            </>
        }
    }
}
