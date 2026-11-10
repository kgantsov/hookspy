use futures_util::stream::AbortHandle;
use futures_util::stream::Abortable;
use gloo_net::http::Request;
use gloo_timers::future::TimeoutFuture;
use wasm_bindgen_futures::spawn_local;
use web_sys::window;
use yew::prelude::*;

use futures_util::StreamExt;
use gloo_net::websocket::{Message, futures::WebSocket};

use crate::components::toast::Toast;
use crate::components::tooltip::Tooltip;
use crate::components::webhook_request_details::WebhookRequest;
use crate::components::webhook_request_details::WebhookRequestDetails;

#[derive(Properties, PartialEq)]
pub struct WebhookRequestListProps {
    pub webhook_id: String,
}

fn websocket_url(path: String) -> String {
    let window = window().expect("no window");
    let location = window.location();

    let protocol = location.protocol().unwrap(); // "http:" or "https:"
    let host = location.host().unwrap(); // "example.com:443"

    let ws_protocol = if protocol == "https:" { "wss" } else { "ws" };

    format!("{ws_protocol}://{host}{path}")
}

fn request_matches(request: &WebhookRequest, query: &str) -> bool {
    if query.is_empty() {
        return true;
    }
    let q = query.to_lowercase();
    request.method.to_lowercase().contains(q.as_str())
        || request.body.to_lowercase().contains(q.as_str())
        || request.headers.to_lowercase().contains(q.as_str())
        || request
            .caller_ip
            .as_deref()
            .map(|ip| ip.to_lowercase().contains(q.as_str()))
            .unwrap_or(false)
}

#[component]
pub fn WebhookRequestList(props: &WebhookRequestListProps) -> Html {
    let webhook_requests = use_state(|| vec![]);
    let webhook_id = props.webhook_id.clone();
    let search_query = use_state(|| String::new());
    let show_toast = use_state(|| false);
    let input_ref = use_node_ref();

    // Reset the query and focus the input whenever the viewed webhook changes.
    {
        let search_query = search_query.clone();
        let input_ref = input_ref.clone();
        use_effect_with(webhook_id.clone(), move |_| {
            search_query.set(String::new());
            if let Some(input) = input_ref.cast::<web_sys::HtmlInputElement>() {
                let _ = input.focus();
            }
            || ()
        });
    }

    {
        let webhook_requests = webhook_requests.clone();
        let webhook_id = props.webhook_id.clone();
        use_effect_with(webhook_id.clone(), move |_| {
            let webhook_requests = webhook_requests.clone();
            let webhook_id = webhook_id.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let resp = Request::get(format!("/api/webhooks/{}/requests", webhook_id).as_str())
                    .send()
                    .await;
                match resp {
                    Ok(resp) => {
                        if resp.status() == 401 {
                            if let Some(win) = window() {
                                let _ = win.location().set_href("/");
                            }
                        } else {
                            let fetched_webhook_requests: Result<Vec<WebhookRequest>, _> =
                                resp.json().await;
                            match fetched_webhook_requests {
                                Ok(fetched_webhook_requests) => {
                                    webhook_requests.set(fetched_webhook_requests)
                                }
                                Err(err) => web_sys::console::log_1(
                                    &format!("Error fetching webhook requests: {}", err).into(),
                                ),
                            }
                        }
                    }
                    Err(err) => {
                        web_sys::console::log_1(&format!("Error fetching webhook: {}", err).into())
                    }
                }
            });
            || ()
        });
    }

    {
        let webhook_requests = webhook_requests.clone();
        let webhook_id = props.webhook_id.clone();

        use_effect_with(webhook_id.clone(), move |current_webhook_id| {
            let (abort_handle, abort_registration) = AbortHandle::new_pair();

            let ws_url =
                websocket_url(format!("/ws/webhooks/{}/notifications", current_webhook_id));
            let webhook_id_for_async = current_webhook_id.clone();
            let webhook_requests = webhook_requests.clone();

            let future = async move {
                let ws = WebSocket::open(&ws_url).unwrap();
                let (_write, mut read) = ws.split();

                while let Some(msg) = read.next().await {
                    match msg {
                        Ok(Message::Text(_)) => {
                            if let Ok(resp) = Request::get(
                                format!("/api/webhooks/{}/requests", webhook_id_for_async).as_str(),
                            )
                            .send()
                            .await
                            {
                                if resp.status() == 401 {
                                    if let Some(win) = window() {
                                        let _ = win.location().set_href("/");
                                    }
                                } else if let Ok(data) = resp.json().await {
                                    webhook_requests.set(data);
                                }
                            }
                        }
                        Err(err) => web_sys::console::error_1(&err.to_string().into()),
                        _ => {}
                    }
                }
            };

            let abortable_future = Abortable::new(future, abort_registration);

            wasm_bindgen_futures::spawn_local(async move {
                // Ignore the result (it returns Err(Aborted) if cancelled, which is expected)
                let _ = abortable_future.await;
            });

            // Cleanup: Abort the task on unmount or ID change
            move || {
                abort_handle.abort();
            }
        });
    }

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

    let on_delete_webhook_requests = {
        let webhook_requests = webhook_requests.clone();
        let show_toast = show_toast.clone();
        Callback::from(move |_| {
            let Some(win) = window() else {
                return;
            };

            let confirmation =
                win.confirm_with_message("Are you sure you want to delete all requests?");
            if !matches!(confirmation, Ok(true)) {
                return;
            }

            let webhook_id = webhook_id.clone();
            let webhook_requests = webhook_requests.clone();
            let show_toast = show_toast.clone();
            spawn_local(async move {
                let req = Request::delete(&format!("/api/webhooks/{}/requests", webhook_id))
                    .send()
                    .await;

                match req {
                    Ok(resp) => {
                        if resp.status() == 401 {
                            if let Some(win) = window() {
                                let _ = win.location().set_href("/");
                            }
                        } else if resp.ok() {
                            webhook_requests.set(vec![]);
                            show_toast.set(true);
                            TimeoutFuture::new(2_000).await;
                            show_toast.set(false);
                        } else {
                            web_sys::console::error_1(
                                &format!("Failed to delete webhook requests: {}", resp.status())
                                    .into(),
                            );
                        }
                    }
                    Err(err) => {
                        web_sys::console::error_1(&err.to_string().into());
                    }
                }
            });
        })
    };

    let filtered: Vec<&WebhookRequest> = webhook_requests
        .iter()
        .filter(|r| request_matches(r, &search_query))
        .collect();

    let count = filtered.len();
    let total = webhook_requests.len();

    let has_query = !search_query.is_empty();
    let search_box_class = if has_query {
        "search-box has-value"
    } else {
        "search-box"
    };

    html! {
        <>
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
                <Tooltip text="Clear all requests">
                    <button
                        class="copy-btn"
                        onclick={on_delete_webhook_requests}
                        aria-label="Clear all requests"
                    >
                        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                            <path d="M11 12H3"/>
                            <path d="M16 6H3"/>
                            <path d="M16 18H3"/>
                            <path d="m19 10-4 4"/>
                            <path d="m15 10 4 4"/>
                        </svg>
                    </button>
                </Tooltip>
            </div>

            <div class="requests-list" key={props.webhook_id.clone()}>
                if !search_query.is_empty() {
                    <div class="search-results-info">
                        { format!("{} {} of {}", count, if count == 1 { "result" } else { "results" }, total) }
                    </div>
                }
                if !search_query.is_empty() && count == 0 {
                    <div class="empty-state">
                        <div class="empty-icon">{ "🔍" }</div>
                        <h3>{ "No matching requests" }</h3>
                        <p>{ "Try a different search term." }</p>
                    </div>
                }
                { for filtered.iter().map(|request| html! {
                    <WebhookRequestDetails
                        request={(*request).clone()}
                        search_query={(*search_query).clone()}
                    />
                }) }
            </div>

            <Toast message="All requests deleted!" visible={*show_toast} />
        </>
    }
}
