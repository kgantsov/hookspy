use gloo_net::http::Request;
use yew::prelude::*;

use hookspy_ui::components::webhook_details::WebhookDetails;
use hookspy_ui::components::webhook_list::Webhook;
use hookspy_ui::components::webhook_list::WebhookList;

#[component]
fn App() -> Html {
    let webhooks = use_state(|| vec![]);
    {
        let webhooks = webhooks.clone();
        use_effect_with((), move |_| {
            let webhooks = webhooks.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_webhooks: Vec<Webhook> = Request::get("/api/webhooks")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                webhooks.set(fetched_webhooks);
            });
            || ()
        });
    }

    let selected_webhook = use_state(|| None);

    let on_webhook_select = {
        let selected_webhook = selected_webhook.clone();
        Callback::from(move |webhook: Webhook| selected_webhook.set(Some(webhook)))
    };

    html! {
        <div class="container">
            <header>
                <div class="logo">
                    <div class="logo-icon">{ "🪝" }</div>
                    <span>{ "HookSpy" }</span>
                </div>
                // <button class="btn btn-primary">
                //     {"+ New Webhook"}
                // </button>
            </header>
            <div class="layout">
                <aside class="sidebar">
                    <div class="sidebar-header">
                        <h2 class="sidebar-title">{ "Webhooks" }</h2>
                        <span class="sidebar-title" style="font-weight: 400"
                            >{webhooks.len()}</span
                        >
                    </div>

                    <WebhookList webhooks={(*webhooks).clone()} on_click={on_webhook_select} />
                </aside>
                <main class="main-content">

                    if let Some(webhook) = &*selected_webhook {
                        <WebhookDetails webhook={webhook.clone()} />
                    }

                </main>
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
