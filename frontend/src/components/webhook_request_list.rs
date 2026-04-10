use futures_util::stream::AbortHandle;
use futures_util::stream::Abortable;
use gloo_net::http::Request;
use web_sys::window;
use yew::prelude::*;

use futures_util::StreamExt;
use gloo_net::websocket::{Message, futures::WebSocket};

use crate::components::webhook_request_details::WebhookRequest;
use crate::components::webhook_request_details::WebhookRequestDetails;

#[derive(Properties, PartialEq)]
pub struct WebhookRequestListProps {
    pub webhook_id: String,
    pub search_query: String,
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

    let search_query = props.search_query.clone();

    let filtered: Vec<&WebhookRequest> = webhook_requests
        .iter()
        .filter(|r| request_matches(r, &search_query))
        .collect();

    let count = filtered.len();
    let total = webhook_requests.len();

    html! {
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
                    search_query={search_query.clone()}
                />
            }) }
        </div>
    }
}
