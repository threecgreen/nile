use yew::prelude::*;

pub struct Header {}

impl Component for Header {
    type Properties = ();
    type Message = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="in-game-header">
                <h1 class="in-game-title">{ "nile" }</h1>
            </div>
        }
    }
}
