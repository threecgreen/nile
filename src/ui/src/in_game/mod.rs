mod board;
mod controls;
mod game;
mod header;
mod player;
mod state;

use yew::prelude::*;

use crate::components::{Button, Container, Modal};
use game::Game;
use header::Header;

pub struct InGame {
    props: Props,
}

#[derive(Clone, Properties)]
pub struct Props {
    pub player_names: Vec<String>,
    pub cpu_player_count: u8,
    pub should_show_shortcuts: bool,
    pub on_new_game: Callback<()>,
    pub on_shortcuts_modal: Callback<bool>,
}

impl Component for InGame {
    type Properties = Props;
    type Message = ();

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        todo!()
    }

    fn view(&self) -> Html {
        let show_shortcuts_modal = self.props.on_shortcuts_modal.reform(|_| true);
        html! {
            <Container>
                <Header />
                <Button title="New game"
                    class=classes!("river-turquoise-bg")
                    // TODO: confirm starting new game
                    on_click={ self.props.on_new_game.clone() }
                >
                    { "New game" }
                </Button>
                <Button title="Shortcuts help"
                    on_click={ show_shortcuts_modal }
                >
                    { "Shortcuts help" }
                </Button>
                { self.view_shortcuts_help_modal() }
                <Game player_names={ self.props.player_names.clone() }
                    cpu_player_count={ self.props.cpu_player_count }
                />
            </Container>
        }
    }
}

impl InGame {
    fn view_shortcuts_help_modal(&self) -> Html {
        const SHORTCUT_BINDINGS: [(&str, &str); 8] = [
            ("q", "rotate counter-clockwise"),
            ("e", "rotate clockwise"),
            ("x", "remove tile"),
            ("u", "undo"),
            ("r", "redo"),
            ("E", "end turn"),
            ("C", "can’t play"),
            ("ESC", "dismiss modal"),
        ];

        if self.props.should_show_shortcuts {
            let dismiss = self.props.on_shortcuts_modal.reform(|_| false);
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
