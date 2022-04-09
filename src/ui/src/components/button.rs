use yew::prelude::*;

pub struct Button {}

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    pub on_click: Callback<()>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or(true)]
    pub is_enabled: bool,
    #[prop_or_default]
    pub title: &'static str,
    #[prop_or_default]
    pub aria_label: &'static str,
    #[prop_or_default]
    pub children: Children,
}

impl Component for Button {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_click = ctx.props().on_click.reform(move |e: MouseEvent| {
            e.prevent_default();
        });
        html! {
            <button onclick={ on_click }
                class={ ctx.props().class.clone() }
                disabled={ !ctx.props().is_enabled }
                title={ ctx.props().title }
                aria-label={ ctx.props().aria_label }
            >
                { ctx.props().children.clone() }
            </button>
        }
    }
}
