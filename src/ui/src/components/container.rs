use yew::prelude::*;

pub struct Container {}

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
}

impl Component for Container {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="container">
                { ctx.props().children.clone() }
            </div>
        }
    }
}
