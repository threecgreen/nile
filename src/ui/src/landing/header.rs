use yew::prelude::*;

use super::cover_art::CoverArt;

pub struct Header {}

impl Component for Header {
    type Properties = ();
    type Message = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
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
