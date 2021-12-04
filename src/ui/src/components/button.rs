use yew::prelude::*;

use super::utils::update_if_changed;

pub struct Button {
    props: Props,
}

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

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        update_if_changed(&mut self.props, props)
    }

    fn view(&self) -> Html {
        let on_click = self.props.on_click.reform(move |e: MouseEvent| {
            e.prevent_default();
        });
        html! {
            <button onclick={ on_click }
                class={ self.props.class.clone() }
                disabled={ !self.props.is_enabled }
                title={ self.props.title }
                aria-label={ self.props.aria_label }
            >
                { self.props.children.clone() }
            </button>
        }
    }
}
