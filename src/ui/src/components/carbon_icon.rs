use yew::prelude::*;

pub struct CarbonIcon {}

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
}

impl Component for CarbonIcon {
    type Properties = Props;
    type Message = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <img class="carbon-icon"
                src={ format!("/icons/{}/{}.svg", ctx.props().size.to_path(), ctx.props().name) }
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
