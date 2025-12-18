use crate::components::create_webhook_modal::CreateWebhookModal;
use crate::routes::Route;
use gloo_net::http::Request;
use web_sys::window;
use yew::html::ChildrenProps;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::webhook_list::Webhook;
use crate::components::webhook_list::WebhookList;

#[component]
pub fn WebhooksLayout(props: &ChildrenProps) -> Html {
    let navigator = use_navigator().unwrap();

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
        let navigator = navigator.clone();
        Callback::from(move |webhook: Webhook| {
            if let Some(window) = window() {
                let confirmation =
                    window.confirm_with_message("Are you sure you want to delete this webhook?");
                if let Ok(true) = confirmation {
                    let selected_webhook = selected_webhook.clone();

                    let webhooks = webhooks.clone();
                    let navigator = navigator.clone();
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
                            navigator.push(&Route::Webhooks);
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
                        <span>
                            <Link<Route> to={Route::Webhooks}>
                                { "HookSpy" }
                            </Link<Route>>
                        </span>

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

                        { props.children.clone() } // ← routed content

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
