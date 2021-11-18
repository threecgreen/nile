use crate::state::GameStore;
use std::rc::Rc;

use yew::prelude::*;
// use yew::services::KeyboardService;

pub struct Game {
    store: GameStore,
    link: ComponentLink<Self>,
    // keyboard_service: KeyboardService,
}

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    pub player_names: Vec<String>,
    pub cpu_player_count: u8,
}

impl Component for Game {
    type Properties = Props;
    type Message = Msg;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            state: Rc::new(State {
                nile: nile::Nile::new(props.player_names, props.cpu_player_count as usize)
                    // TODO: remove expect
                    .expect("nile"),
                current_turn_tiles: HashSet::new(),
            }),
            link,
            keyboard_service: KeyboardService {},
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        todo!()
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        todo!()
    }

    fn view(&self) -> Html {
        html! {
            <>
                <section>
                </section>
                <section>
                </section>
            </>
        }
    }
}
