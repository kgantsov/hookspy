use crate::components::create_webhook_modal::CreateWebhookModal;
use crate::routes::Route;

use futures_util::StreamExt;
use futures_util::stream::AbortHandle;
use futures_util::stream::Abortable;
use gloo_net::http::Request;
use gloo_net::websocket::{Message, futures::WebSocket};
use web_sys::window;
use yew::html::ChildrenProps;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::theme_switcher::ThemeSwitcher;
use crate::components::tooltip::Tooltip;
use crate::components::webhook_list::Webhook;
use crate::components::webhook_list::WebhookList;

fn websocket_url(path: &str) -> String {
    let window = window().expect("no window");
    let location = window.location();
    let protocol = location.protocol().unwrap();
    let host = location.host().unwrap();
    let ws_protocol = if protocol == "https:" { "wss" } else { "ws" };
    format!("{ws_protocol}://{host}{path}")
}

#[component]
pub fn WebhooksLayout(props: &ChildrenProps) -> Html {
    let navigator = use_navigator().unwrap();
    let route = use_route::<Route>();
    let selected_webhook_id = match &route {
        Some(Route::Webhook { webhook_id }) => Some(webhook_id.clone()),
        _ => None,
    };

    let webhooks = use_state(|| vec![]);

    // Mirrors kept in sync so the long-lived WS async loop can always read current values
    // without being re-created on every state change.
    let webhooks_ref = use_mut_ref(|| vec![]);
    let selected_id_ref = use_mut_ref(|| None::<String>);

    // Keep the selected-id mirror in sync with the derived route value.
    {
        let selected_id_ref = selected_id_ref.clone();
        let selected_webhook_id = selected_webhook_id.clone();
        use_effect_with(selected_webhook_id.clone(), move |id| {
            *selected_id_ref.borrow_mut() = id.clone();
            || ()
        });
    }

    // Keep the webhooks mirror in sync whenever the state changes.
    {
        let webhooks_ref = webhooks_ref.clone();
        let webhooks = webhooks.clone();
        use_effect_with((*webhooks).clone(), move |current| {
            *webhooks_ref.borrow_mut() = current.clone();
            || ()
        });
    }

    let fetch_webhooks = {
        let webhooks = webhooks.clone();
        Callback::from(move |_: ()| {
            let webhooks = webhooks.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let resp = Request::get("/api/webhooks").send().await;

                match resp {
                    Ok(resp) => {
                        if resp.status() == 401 {
                            if let Some(win) = window() {
                                let _ = win.location().set_href("/");
                            }
                        } else {
                            let fetched_webhooks: Result<Vec<Webhook>, _> = resp.json().await;
                            match fetched_webhooks {
                                Ok(fetched_webhooks) => webhooks.set(fetched_webhooks),
                                Err(err) => web_sys::console::log_1(
                                    &format!("Error fetching webhooks: {}", err).into(),
                                ),
                            }
                        }
                    }
                    Err(err) => {
                        web_sys::console::log_1(&format!("Error fetching webhooks: {}", err).into())
                    }
                }
            });
        })
    };

    // Initial fetch on mount.
    {
        let fetch_webhooks = fetch_webhooks.clone();
        use_effect_with((), move |_| {
            fetch_webhooks.emit(());
            || ()
        });
    }

    // When the user navigates to a webhook, optimistically clear has_unread in local state
    // and re-fetch the list so the seen timestamp is reflected from the server.
    {
        let webhooks = webhooks.clone();
        let fetch_webhooks = fetch_webhooks.clone();
        use_effect_with(selected_webhook_id.clone(), move |current_id| {
            if let Some(id) = current_id.clone() {
                let updated: Vec<Webhook> = (*webhooks)
                    .iter()
                    .map(|w| {
                        if w.id == id {
                            Webhook {
                                has_unread: false,
                                ..w.clone()
                            }
                        } else {
                            w.clone()
                        }
                    })
                    .collect();
                webhooks.set(updated);
            }
            fetch_webhooks.emit(());
            || ()
        });
    }

    // Single user-level WS subscription opened once on mount.
    // Reads current list and selected id through refs so the loop is never re-created.
    {
        let webhooks = webhooks.clone();
        let webhooks_ref = webhooks_ref.clone();
        let selected_id_ref = selected_id_ref.clone();
        use_effect_with((), move |_| {
            let (abort_handle, abort_registration) = AbortHandle::new_pair();

            let ws_url = websocket_url("/ws/user/notifications");

            let future = async move {
                match WebSocket::open(&ws_url) {
                    Ok(ws) => {
                        let (_write, mut read) = ws.split();
                        while let Some(msg) = read.next().await {
                            match msg {
                                Ok(Message::Text(webhook_id)) => {
                                    // Don't mark as unread if the user is already viewing it.
                                    let is_selected = selected_id_ref
                                        .borrow()
                                        .as_deref()
                                        .map(|id| id == webhook_id)
                                        .unwrap_or(false);

                                    if !is_selected {
                                        // Read the current list from the ref (always up-to-date).
                                        let updated: Vec<Webhook> = webhooks_ref
                                            .borrow()
                                            .iter()
                                            .map(|w| {
                                                if w.id == webhook_id {
                                                    Webhook {
                                                        has_unread: true,
                                                        ..w.clone()
                                                    }
                                                } else {
                                                    w.clone()
                                                }
                                            })
                                            .collect();
                                        webhooks.set(updated);
                                    }
                                }
                                Err(err) => {
                                    web_sys::console::error_1(&err.to_string().into());
                                }
                                _ => {}
                            }
                        }
                    }
                    Err(err) => {
                        web_sys::console::error_1(
                            &format!("Failed to open user notifications WS: {:?}", err).into(),
                        );
                    }
                }
            };

            let abortable = Abortable::new(future, abort_registration);
            wasm_bindgen_futures::spawn_local(async move {
                let _ = abortable.await;
            });

            move || {
                abort_handle.abort();
            }
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
        let fetch_webhooks_for_delete = fetch_webhooks.clone();
        let navigator = navigator.clone();
        Callback::from(move |webhook: Webhook| {
            if let Some(window) = window() {
                let confirmation =
                    window.confirm_with_message("Are you sure you want to delete this webhook?");
                if let Ok(true) = confirmation {
                    let selected_webhook = selected_webhook.clone();
                    let fetch_webhooks_for_delete = fetch_webhooks_for_delete.clone();
                    let navigator = navigator.clone();
                    wasm_bindgen_futures::spawn_local(async move {
                        let response = Request::delete(&format!("/api/webhooks/{}", webhook.id))
                            .send()
                            .await;
                        if response.is_ok() {
                            match response {
                                Ok(response) => {
                                    if response.status() == 401 {
                                        let _ = window.location().set_href("/");
                                    }

                                    selected_webhook.set(None);
                                    fetch_webhooks_for_delete.emit(());
                                    navigator.push(&Route::Webhooks);
                                }
                                Err(_) => {
                                    selected_webhook.set(None);
                                }
                            }
                        }
                    });
                }
            }
        })
    };

    let on_logout = Callback::from(move |_: MouseEvent| {
        wasm_bindgen_futures::spawn_local(async move {
            let _ = Request::post("/api/auth/logout").send().await;
            if let Some(win) = window() {
                let _ = win.location().set_href("/");
            }
        });
    });

    html! {
        <>
            <div class="container">
                <header>
                    <div class="logo">
                        <div class="logo-icon">
                            <svg width="20" height="20" viewBox="0 0 20 20" fill="none">
                                <circle cx="7" cy="5" r="2.5" stroke="white" stroke-width="1.75"/>
                                <circle cx="7" cy="5" r="1" fill="white"/>
                                <path d="M7 7.5 L7 12 Q7 17 12 17 Q17 17 17 12 Q17 9.5 14.5 9.5" stroke="white" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                            </svg>
                        </div>
                        <span>
                            <Link<Route> to={Route::Webhooks}>
                                { "HookSpy" }
                            </Link<Route>>
                        </span>
                    </div>
                    <div class="header-actions">
                        <ThemeSwitcher />
                        <Tooltip text="Sign out">
                            <button
                                class="icon-btn"
                                onclick={on_logout}
                                aria-label="Sign out"
                            >
                                <svg width="16" height="16" viewBox="0 0 16 16" fill="none">
                                    <path d="M6 2H3a1 1 0 0 0-1 1v10a1 1 0 0 0 1 1h3" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
                                    <path d="M10 5l3 3-3 3" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
                                    <path d="M13 8H6" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
                                </svg>
                            </button>
                        </Tooltip>
                    </div>
                </header>
                <div class="layout">
                    <aside class="sidebar">
                        <div class="sidebar-header">
                            <h2 class="sidebar-title">{ "Webhooks" }</h2>
                            <span class="sidebar-count">
                                {webhooks.len()}
                            </span>
                        </div>

                        <button
                            class="sidebar-new-btn"
                            onclick={
                                let create_webhook_modal_is_open = create_webhook_modal_is_open.clone();
                                move |_| create_webhook_modal_is_open.set(true)
                            }
                        >
                            <svg width="14" height="14" viewBox="0 0 14 14" fill="none">
                                <path d="M7 1v12M1 7h12" stroke="currentColor" stroke-width="1.75" stroke-linecap="round"/>
                            </svg>
                            { "New Webhook" }
                        </button>

                        <WebhookList
                            webhooks={(*webhooks).clone()}
                            on_click={on_webhook_select}
                            on_delete={on_webhook_delete}
                            selected_webhook_id={selected_webhook_id}
                        />
                    </aside>
                    <main class="main-content">
                        { props.children.clone() }
                    </main>
                </div>
            </div>

            <CreateWebhookModal
                is_open={*create_webhook_modal_is_open}
                on_close={
                    let create_webhook_modal_is_open = create_webhook_modal_is_open.clone();
                    let fetch_webhooks_on_close = fetch_webhooks.clone();
                    move |_| {
                        create_webhook_modal_is_open.set(false);
                        fetch_webhooks_on_close.emit(());
                    }
                }
            />
        </>
    }
}
