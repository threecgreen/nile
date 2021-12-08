use yew::prelude::*;

use crate::components::utils::update_if_changed;

pub use click::Button as ClickButton;
pub use link::Button as LinkButton;

pub mod link {
    use super::*;

    pub struct Button {
        props: Props,
    }

    #[derive(Clone, Properties, PartialEq)]
    pub struct Props {
        pub href: &'static str,
        #[prop_or_default]
        pub children: Children,
    }

    impl Component for Button {
        type Properties = Props;
        type Message = ();

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
            html! {
                <a href={ self.props.href }
                    class=classes!("landing-button", "nile-blue-bg")
                >
                    { self.props.children.clone() }
                </a>
            }
        }
    }
}

pub mod click {
    use crate::components::Button as Btn;

    use super::*;

    pub struct Button {
        props: Props,
    }

    #[derive(Clone, Properties, PartialEq)]
    pub struct Props {
        pub on_click: Callback<()>,
        #[prop_or_default]
        pub children: Children,
    }

    impl Component for Button {
        type Properties = Props;
        type Message = ();

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
            html! {
                <Btn on_click={ self.props.on_click.clone() }
                    class={ classes!("landing-button", "river-turquoise-bg") }
                >
                    { self.props.children.clone() }
                </Btn>
            }
        }
    }
}
