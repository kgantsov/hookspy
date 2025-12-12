use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_futures::spawn_local;
use web_sys::window;
use yew::prelude::*;

use crate::components::{webhook_list::Webhook, webhook_request_list::WebhookRequestList};

#[derive(Properties, PartialEq)]
pub struct WebhookDetailsProps {
    pub webhook: Webhook,
}

#[component]
pub fn WebhookDetails(WebhookDetailsProps { webhook }: &WebhookDetailsProps) -> Html {
    let url = format!("http://0.0.0.0:3000/api/webhooks/{}", webhook.id);
    let url_to_copy = url.clone();

    let onclick = Callback::from(move |_| {
        let text = url_to_copy.clone();
        spawn_local(async move {
            if let Some(window) = window() {
                let navigator = window.navigator();
                let clipboard = navigator.clipboard();

                // Convert JS Promise into a Rust Future
                let promise = clipboard.write_text(&text);
                match JsFuture::from(promise).await {
                    Ok(_) => web_sys::console::log_1(&"Copied!".into()),
                    Err(err) => web_sys::console::log_1(&format!("Copy failed: {:?}", err).into()),
                }
            }
        });
    });

    html! {
        <>
            <div class="content-header" key={webhook.id.clone()}>
                <div>
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

            <WebhookRequestList webhook_id={webhook.id.clone()} />
        </>
    }
}
