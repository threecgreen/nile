use yew::prelude::*;

use super::utils::update_if_changed;

pub struct CarbonIcon {
    props: Props,
}

#[derive(Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum Size {
    S16,
    S24,
}

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    pub name: &'static str,
    pub size: Size,
    #[prop_or_default]
    pub aria_label: &'static str,
}

impl Component for CarbonIcon {
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
            <img src={ format!("/icons/{}/{}.svg", self.props.size.to_path(), self.props.name) }
                alt={ self.props.aria_label }
            />
        }
    }
}

impl Size {
    fn to_path(self) -> &'static str {
        match self {
            Size::S16 => "16",
            Size::S24 => "24",
        }
    }
}
