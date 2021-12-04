use yew::prelude::*;

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
            <div class="in-game-header">
                <h1 class="in-game-title">{ "nile" }</h1>
            </div>
        }
    }
}
