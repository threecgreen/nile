use yew::prelude::*;

use super::cover_art::CoverArt;

pub struct Header {}

impl Component for Header {
    type Properties = ();
    type Message = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="landing-header">
                <CoverArt />
                <div class="header-text">
                    <h1 class="landing-title">{ "nile" }</h1>
                    <h2 class="subtitle">{ "a path-creating game" }</h2>
                </div>
            </div>
        }
    }
}
