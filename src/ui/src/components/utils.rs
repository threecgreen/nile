use yew::{html, virtual_dom::VNode, Html};

pub fn update_if_changed<T: PartialEq>(prev: &mut T, new: T) -> bool {
    if *prev == new {
        false
    } else {
        *prev = new;
        true
    }
}

pub fn if_render<T: std::fmt::Display>(predicate: bool, content: T) -> Html {
    if predicate {
        html! {content}
    } else {
        html! {}
    }
}

pub fn if_render_html(predicate: bool, content: VNode) -> Html {
    if predicate {
        content
    } else {
        html! {}
    }
}
