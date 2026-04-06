use web_sys::window;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ToastProps {
    pub message: AttrValue,
    pub visible: bool,
}

#[component]
pub fn Toast(props: &ToastProps) -> Html {
    let portal_host = window()
        .and_then(|w| w.document())
        .and_then(|d| d.body())
        .map(|b| b.into());

    let Some(host) = portal_host else {
        return html! {};
    };

    let visibility_class = if props.visible {
        "toast toast-visible"
    } else {
        "toast"
    };

    yew::create_portal(
        html! {
            <div class={visibility_class}>
                <span class="toast-icon">{"✓"}</span>
                <span class="toast-message">{ props.message.clone() }</span>
            </div>
        },
        host,
    )
}
