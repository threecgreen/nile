use yew::prelude::*;

pub struct Modal {}

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub children: Children,
}

impl Component for Modal {
    type Properties = Props;
    type Message = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="modal">
                <div class="modal-content">
                    { ctx.props().children.clone() }
                </div>
            </div>
        }
    }
}

pub mod error {
    use crate::components::button::Button;

    use super::*;

    pub struct ErrorModal {}

    #[derive(Properties, Clone, PartialEq)]
    pub struct Props {
        pub msg: String,
        pub dismiss: Callback<()>,
    }

    impl Component for ErrorModal {
        type Properties = Props;
        type Message = ();

        fn create(_ctx: &Context<Self>) -> Self {
            Self {}
        }

        fn view(&self, ctx: &Context<Self>) -> Html {
            html! {
                <Modal>
                    <p>{ &ctx.props().msg }</p>
                    <Button title="Dismiss"
                        class={ classes!("nile-blue-bg" ) }
                        on_click={ ctx.props().dismiss.clone() }
                    >
                        { "Dismiss" }
                    </Button>
                </Modal>
            }
        }
    }
}
