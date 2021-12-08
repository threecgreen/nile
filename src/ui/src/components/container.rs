use yew::prelude::*;

use super::utils::update_if_changed;

pub struct Container {
    props: Props,
}

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
}

impl Component for Container {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        update_if_changed(&mut self.props, props)
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="container">
                { self.props.children.clone() }
            </div>
        }
    }
}
