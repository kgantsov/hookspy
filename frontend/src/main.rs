use gloo_net::http::Request;
use hookspy_ui::components::create_webhook_modal::CreateWebhookModal;
use web_sys::window;
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

    let create_webhook_modal_is_open = use_state(|| false);

    let selected_webhook = use_state(|| None);

    let on_webhook_select = {
        let selected_webhook = selected_webhook.clone();
        Callback::from(move |webhook: Webhook| {
            selected_webhook.set(Some(webhook));
        })
    };

    let on_webhook_delete = {
        let selected_webhook = selected_webhook.clone();
        let webhooks = webhooks.clone();
        Callback::from(move |webhook: Webhook| {
            if let Some(window) = window() {
                let confirmation =
                    window.confirm_with_message("Are you sure you want to delete this webhook?");
                if let Ok(true) = confirmation {
                    let selected_webhook = selected_webhook.clone();
                    let webhooks = webhooks.clone();
                    wasm_bindgen_futures::spawn_local(async move {
                        let response = Request::delete(&format!("/api/webhooks/{}", webhook.id))
                            .send()
                            .await;
                        if response.is_ok() {
                            selected_webhook.set(None);

                            let fetched_webhooks: Vec<Webhook> = Request::get("/api/webhooks")
                                .send()
                                .await
                                .unwrap()
                                .json()
                                .await
                                .unwrap();
                            webhooks.set(fetched_webhooks);
                        }
                    });
                }
            }
        })
    };

    html! {
        <>
            <div class="container">
                <header>
                    <div class="logo">
                        <div class="logo-icon">{ "🪝" }</div>
                        <span>{ "HookSpy" }</span>
                    </div>
                    <button
                    class="btn btn-primary"
                    onclick={
                        let create_webhook_modal_is_open = create_webhook_modal_is_open.clone();
                        move |_| create_webhook_modal_is_open.set(true)
                    }>
                        {"+ New Webhook"}
                    </button>
                </header>
                <div class="layout">
                    <aside class="sidebar">
                        <div class="sidebar-header">
                            <h2 class="sidebar-title">{ "Webhooks" }</h2>
                            <span class="sidebar-title" style="font-weight: 400"
                                >{webhooks.len()}</span
                            >
                        </div>

                        <WebhookList
                            webhooks={(*webhooks).clone()}
                            on_click={on_webhook_select}
                            on_delete={on_webhook_delete}
                        />
                    </aside>
                    <main class="main-content">

                        if let Some(webhook) = &*selected_webhook {
                            <WebhookDetails webhook={webhook.clone()} />
                        }

                    </main>
                </div>
            </div>

            <CreateWebhookModal
                is_open={*create_webhook_modal_is_open}
                on_close={
                    let create_webhook_modal_is_open = create_webhook_modal_is_open.clone();
                    move |_| {create_webhook_modal_is_open.set(false);
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
                        });}
                }
            />
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
