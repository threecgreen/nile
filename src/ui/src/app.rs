use yew::prelude::*;

use crate::{
    components::{utils::update_if_changed, Footer},
    in_game::InGame,
    landing::Landing,
};

pub struct App {
    link: ComponentLink<App>,
    player_names: Vec<String>,
    has_confirmed: bool,
    cpu_player_count: u8,
    game_number: u32,
    should_show_shortcuts: bool,
    should_show_new_game_form: bool,
}

pub enum Msg {
    AddPlayer,
    PlayerNameChange(PlayerNameChange),
    RemovePlayer,
    AddCpuPlayer,
    RemoveCpuPlayer,
    Confirm,
    SetShouldShowShortcuts(bool),
    SetShouldShowNewGameForm(bool),
    NewGame,
    Reset,
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
            should_show_shortcuts: false,
            should_show_new_game_form: false,
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
            Msg::SetShouldShowShortcuts(should_show_shortcuts) => {
                update_if_changed(&mut self.should_show_shortcuts, should_show_shortcuts)
            }
            Msg::SetShouldShowNewGameForm(should_show_new_game_form) => update_if_changed(
                &mut self.should_show_new_game_form,
                should_show_new_game_form,
            ),
            Msg::NewGame => {
                self.has_confirmed = false;
                self.game_number += 1;
                true
            }
            Msg::Reset => {
                self.player_names = vec![String::default()];
                self.cpu_player_count = 1;
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let dispatch = self.link.callback(|action| action);
        let on_new_game = self.link.callback(|_| Msg::NewGame);
        let on_shortcuts_modal = self.link.callback(Msg::SetShouldShowShortcuts);
        html! {
            <div id="app-container">
                <main>{ if self.has_confirmed { html! {
                    <InGame player_names={ self.player_names.clone() }
                        cpu_player_count={ self.cpu_player_count }
                        should_show_shortcuts={ self.should_show_shortcuts }
                        on_new_game={ on_new_game }
                        on_shortcuts_modal={ on_shortcuts_modal }
                    />
                } } else { html! {
                    <Landing player_names={ self.player_names.clone() }
                        cpu_player_count={ self.cpu_player_count }
                        should_show_new_game_form={ self.should_show_new_game_form }
                        dispatch={ dispatch }
                    />
                } } }
                </main>
                <footer>
                    <Footer />
                </footer>
            </div>
        }
    }
}
