use gloo_timers::future::TimeoutFuture;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_futures::spawn_local;
use web_sys::window;
use yew::prelude::*;

use crate::components::{
    toast::Toast, webhook_list::Webhook, webhook_request_list::WebhookRequestList,
};

#[derive(Properties, PartialEq)]
pub struct WebhookDetailsProps {
    pub webhook: Webhook,
}

#[component]
pub fn WebhookDetails(WebhookDetailsProps { webhook }: &WebhookDetailsProps) -> Html {
    let url = webhook.url.clone();
    let url_to_copy = url.clone();

    let show_toast = use_state(|| false);
    let search_query = use_state(|| String::new());
    let input_ref = use_node_ref();

    // Reset the query and focus the input whenever the viewed webhook changes.
    {
        let search_query = search_query.clone();
        let input_ref = input_ref.clone();
        use_effect_with(webhook.id.clone(), move |_| {
            search_query.set(String::new());
            if let Some(input) = input_ref.cast::<web_sys::HtmlInputElement>() {
                let _ = input.focus();
            }
            || ()
        });
    }

    let onclick = {
        let show_toast = show_toast.clone();
        Callback::from(move |_| {
            let text = url_to_copy.clone();
            let show_toast = show_toast.clone();
            spawn_local(async move {
                if let Some(window) = window() {
                    let navigator = window.navigator();
                    let clipboard = navigator.clipboard();

                    let promise = clipboard.write_text(&text);
                    match JsFuture::from(promise).await {
                        Ok(_) => {
                            web_sys::console::log_1(&"Copied!".into());
                            show_toast.set(true);
                            TimeoutFuture::new(2_000).await;
                            show_toast.set(false);
                        }
                        Err(err) => {
                            web_sys::console::log_1(&format!("Copy failed: {:?}", err).into());
                        }
                    }
                }
            });
        })
    };

    let on_search = {
        let search_query = search_query.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            search_query.set(input.value());
        })
    };

    let on_clear = {
        let search_query = search_query.clone();
        let input_ref = input_ref.clone();
        Callback::from(move |_: MouseEvent| {
            search_query.set(String::new());
            if let Some(input) = input_ref.cast::<web_sys::HtmlInputElement>() {
                let _ = input.focus();
            }
        })
    };

    let has_query = !(*search_query).is_empty();
    let search_box_class = if has_query {
        "search-box has-value"
    } else {
        "search-box"
    };

    html! {
        <>
            <div class="content-header" key={webhook.id.clone()}>
                <div style="width: 100%">
                    <h1 class="content-title">{ &webhook.name.clone() }</h1>
                    <div
                        class="endpoint-display"
                        style="margin-top: 1rem"
                    >
                        <div class="endpoint-url">
                            {url}
                        </div>
                        <button class="copy-btn" onclick={onclick}>{ "Copy URL" }</button>
                    </div>
                </div>
            </div>

            <div class="search-filter">
                <div class={search_box_class}>
                    <span class="search-icon">
                        <svg width="14" height="14" viewBox="0 0 14 14" fill="none" xmlns="http://www.w3.org/2000/svg">
                            <circle cx="6" cy="6" r="4.5" stroke="currentColor" stroke-width="1.5"/>
                            <path d="M9.5 9.5L12.5 12.5" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
                        </svg>
                    </span>
                    <input
                        ref={input_ref}
                        class="search-input"
                        type="text"
                        placeholder="Search requests…"
                        value={(*search_query).clone()}
                        oninput={on_search}
                    />
                    if has_query {
                        <button class="search-clear-btn" onclick={on_clear} type="button" aria-label="Clear search">
                            <svg width="12" height="12" viewBox="0 0 12 12" fill="none" xmlns="http://www.w3.org/2000/svg">
                                <path d="M1 1L11 11M11 1L1 11" stroke="currentColor" stroke-width="1.75" stroke-linecap="round"/>
                            </svg>
                        </button>
                    }
                </div>
            </div>

            <WebhookRequestList webhook_id={webhook.id.clone()} search_query={(*search_query).clone()} />

            <Toast message="Copied to clipboard!" visible={*show_toast} />
        </>
    }
}
