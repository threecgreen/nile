use yew::prelude::*;

pub struct Container {
    props: Props,
}

#[derive(Clone, Properties)]
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
        self.props = props;
        true
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
