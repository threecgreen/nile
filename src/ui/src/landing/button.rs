use yew::prelude::*;

pub use click::Button as ClickButton;
pub use link::Button as LinkButton;

pub mod link {
    use super::*;

    pub struct Button {}

    #[derive(Clone, Properties, PartialEq)]
    pub struct Props {
        pub href: &'static str,
        #[prop_or_default]
        pub children: Children,
    }

    impl Component for Button {
        type Properties = Props;
        type Message = ();

        fn create(_ctx: &Context<Self>) -> Self {
            Self {}
        }

        fn view(&self, ctx: &Context<Self>) -> Html {
            html! {
                <a href={ ctx.props().href }
                    class={ classes!("landing-button", "nile-blue-bg") }
                >
                    { ctx.props().children.clone() }
                </a>
            }
        }
    }
}

pub mod click {
    use crate::components::Button as Btn;

    use super::*;

    pub struct Button {}

    #[derive(Clone, Properties, PartialEq)]
    pub struct Props {
        pub on_click: Callback<()>,
        #[prop_or_default]
        pub children: Children,
    }

    impl Component for Button {
        type Properties = Props;
        type Message = ();

        fn create(_ctx: &Context<Self>) -> Self {
            Self {}
        }

        fn view(&self, ctx: &Context<Self>) -> Html {
            html! {
                <Btn on_click={ ctx.props().on_click.clone() }
                    class={ classes!("landing-button", "river-turquoise-bg") }
                >
                    { ctx.props().children.clone() }
                </Btn>
            }
        }
    }
}
