use web_sys::window;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TooltipProps {
    pub text: AttrValue,
    pub children: Children,
}

#[component]
pub fn Tooltip(props: &TooltipProps) -> Html {
    let visible = use_state(|| false);
    let x = use_state(|| 0.0_f64);
    let y = use_state(|| 0.0_f64);
    let wrapper_ref = use_node_ref();

    let on_mouse_enter = {
        let visible = visible.clone();
        let x = x.clone();
        let y = y.clone();
        let wrapper_ref = wrapper_ref.clone();
        Callback::from(move |_: MouseEvent| {
            if let Some(el) = wrapper_ref.cast::<web_sys::Element>() {
                let rect = el.get_bounding_client_rect();
                let win = window().unwrap();
                let scroll_x = win.scroll_x().unwrap_or(0.0);
                let scroll_y = win.scroll_y().unwrap_or(0.0);
                // Centre horizontally over the wrapper, place above it
                x.set(rect.left() + scroll_x + rect.width() / 2.0);
                y.set(rect.top() + scroll_y);
            }
            visible.set(true);
        })
    };

    let on_mouse_leave = {
        let visible = visible.clone();
        Callback::from(move |_: MouseEvent| {
            visible.set(false);
        })
    };

    // Portal host — document.body
    let portal_host = window()
        .and_then(|w| w.document())
        .and_then(|d| d.body())
        .map(|b| b.into());

    let tooltip_portal = if *visible {
        if let Some(host) = portal_host {
            let style = format!(
                "position:absolute; left:{:.2}px; top:{:.2}px; transform:translate(-50%, calc(-100% - 6px)); \
                 background:#2a2a2a; color:#e8e8e8; font-size:0.72rem; white-space:nowrap; \
                 padding:0.35rem 0.6rem; border-radius:5px; border:1px solid #333; \
                 pointer-events:none; z-index:9999; \
                 font-family:-apple-system,BlinkMacSystemFont,'Segoe UI','Roboto',sans-serif; font-weight:400;",
                *x, *y
            );
            yew::create_portal(html! { <div {style}>{ props.text.clone() }</div> }, host)
        } else {
            html! {}
        }
    } else {
        html! {}
    };

    html! {
        <>
            <span
                ref={wrapper_ref}
                style="position:relative; display:inline-flex;"
                onmouseenter={on_mouse_enter}
                onmouseleave={on_mouse_leave}
            >
                { for props.children.iter() }
            </span>
            { tooltip_portal }
        </>
    }
}
