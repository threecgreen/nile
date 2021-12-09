use yew::prelude::*;

use super::utils::update_if_changed;

pub struct Modal {
    children: Children,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub children: Children,
}

impl Component for Modal {
    type Properties = Props;
    type Message = ();

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {
            children: props.children,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        update_if_changed(&mut self.children, props.children)
    }

    fn view(&self) -> Html {
        html! {
            <div class="modal">
                <div class="modal-content">
                    { self.children.clone() }
                </div>
            </div>
        }
    }
}

pub mod error {
    use crate::components::button::Button;

    use super::*;

    pub struct ErrorModal {
        props: Props,
    }

    #[derive(Properties, Clone, PartialEq)]
    pub struct Props {
        pub msg: String,
        pub dismiss: Callback<()>,
    }

    impl Component for ErrorModal {
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
                <Modal>
                    <p>{ &self.props.msg }</p>
                    <Button title="Dismiss"
                        class=classes!("nile-blue-bg")
                        on_click={ self.props.dismiss.clone() }
                    >
                        { "Dismiss" }
                    </Button>
                </Modal>
            }
        }
    }
}
