use crate::components::{
    utils::update_if_changed, Button, Container, Footer, Game, GameForm, Modal,
};
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
    AddPlayer,
    PlayerNameChange(PlayerNameChange),
    RemovePlayer,
    AddCpuPlayer,
    RemoveCpuPlayer,
    Confirm,
    ShowShortcutsHelp,
    DismissShortcutsHelp,
    NewGame,
}

pub struct PlayerNameChange {
    pub idx: usize,
    pub name: String,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            player_names: vec![String::default()],
            has_confirmed: false,
            cpu_player_count: 1,
            game_number: 1,
            show_shortcuts_modal: false,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddPlayer => {
                let total_player_count = self.player_names.len() + self.cpu_player_count as usize;
                if total_player_count < 4 {
                    self.player_names.push(String::default());
                    true
                } else {
                    false
                }
            }
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
                let cpu_player_count = (self.cpu_player_count + 1).min(3);
                update_if_changed(&mut self.cpu_player_count, cpu_player_count)
            }
            Msg::RemoveCpuPlayer => {
                let cpu_player_count = (self.cpu_player_count - 1).max(0);
                update_if_changed(&mut self.cpu_player_count, cpu_player_count)
            }
            Msg::Confirm => update_if_changed(&mut self.has_confirmed, true),
            Msg::ShowShortcutsHelp => update_if_changed(&mut self.show_shortcuts_modal, true),
            Msg::DismissShortcutsHelp => update_if_changed(&mut self.show_shortcuts_modal, false),
            Msg::NewGame => {
                self.has_confirmed = false;
                self.game_number += 1;
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div id="app-container">
                <main>
                    <Container>
                        <h1>{ "Nile" }</h1>
                        { if self.has_confirmed { self.view_game() } else { self.view_game_form() } }
                    </Container>
                </main>
                <footer>
                    <Footer />
                </footer>
            </div>
        }
    }
}

impl App {
    fn view_game(&self) -> Html {
        let new_game = self.link.callback(|_| Msg::NewGame);
        let show_shortcuts_modal = self.link.callback(|_| Msg::ShowShortcutsHelp);
        html! {
            <>
                <Button title="New game"
                    // TODO: confirm starting new game
                    on_click={ new_game }
                >
                    { "New game" }
                </Button>
                <Button title="Shortcuts help"
                    on_click={ show_shortcuts_modal }
                >
                    { "Shortcuts help" }
                </Button>
                { self.view_shortcuts_help_modal() }
                <Game player_names={ self.player_names.clone() }
                    cpu_player_count={ self.cpu_player_count }
                />
            </>
        }
    }

    fn view_game_form(&self) -> Html {
        let dispatch = self.link.callback(|action| action);
        html! {
            <>
                <h2 class="center-text">{ "New game" }</h2>
                <GameForm player_names={ self.player_names.clone() }
                    cpu_player_count={ self.cpu_player_count }
                    dispatch={ dispatch }
                />
            </>
        }
    }

    fn view_shortcuts_help_modal(&self) -> Html {
        const SHORTCUT_BINDINGS: [(&str, &str); 7] = [
            ("q", "rotate counter-clockwise"),
            ("e", "rotate clockwise"),
            ("x", "remove tile"),
            ("u", "undo"),
            ("r", "redo"),
            ("E", "end turn"),
            ("C", "can’t play"),
        ];

        if self.show_shortcuts_modal {
            let dismiss = self.link.callback(|_| Msg::DismissShortcutsHelp);

            html! {
                <Modal>
                    <h2>{ "Keyboard shortcuts" }</h2>
                    <section>
                        <table class="shortcuts-help">
                            <tbody>
                                { for { SHORTCUT_BINDINGS.iter().map(|(key, help_text)| html!{
                                        <tr key={ *key }>
                                            <td><span class="help-key">{ key }</span></td>
                                            <td>{ help_text }</td>
                                        </tr>
                                }) } }
                                <tr>
                                    <td>
                                        <span class="help-key">{ "1" }</span>
                                        { "–" }
                                        <span class="help-key">{ "5" }</span>
                                    </td>
                                    <td>{ "select the n" }<sup>{ "th" }</sup>{ " tile from the tile rack" }</td>
                                </tr>
                            </tbody>
                        </table>
                    </section>
                    <Button title="Dismiss"
                        on_click={ dismiss }
                    >
                        { "Dismiss" }
                    </Button>
                </Modal>
            }
        } else {
            html! {}
        }
    }
}
