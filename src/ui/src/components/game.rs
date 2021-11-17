use std::rc::Rc;

use yew::prelude::*;

use crate::app::Msg;

pub struct Game {
    nile: Rc<nile::Nile>,
    link: ComponentLink<Self>,
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
            nile: Rc::new(
                nile::Nile::new(props.player_names, props.cpu_player_count as usize).expect("nile"),
            ),
            link,
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
