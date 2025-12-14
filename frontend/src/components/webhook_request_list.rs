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
}

fn websocket_url(path: String) -> String {
    let window = window().expect("no window");
    let location = window.location();

    let protocol = location.protocol().unwrap(); // "http:" or "https:"
    let host = location.host().unwrap(); // "example.com:443"

    let ws_protocol = if protocol == "https:" { "wss" } else { "ws" };

    format!("{ws_protocol}://{host}{path}")
}

#[component]
pub fn WebhookRequestList(
    WebhookRequestListProps { webhook_id }: &WebhookRequestListProps,
) -> Html {
    let webhook_requests = use_state(|| vec![]);
    {
        let webhook_requests = webhook_requests.clone();
        let webhook_id = webhook_id.clone();
        use_effect_with(webhook_id.clone(), move |_| {
            let webhook_requests = webhook_requests.clone();
            let webhook_id = webhook_id.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_webhook_requests: Vec<WebhookRequest> =
                    Request::get(format!("/api/webhooks/{}/requests", webhook_id).as_str())
                        .send()
                        .await
                        .unwrap()
                        .json()
                        .await
                        .unwrap();
                webhook_requests.set(fetched_webhook_requests);
            });
            || ()
        });
    }

    {
        let webhook_requests = webhook_requests.clone();
        let webhook_id = webhook_id.clone();

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
                                if let Ok(data) = resp.json().await {
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

    html! {
        <div class="requests-list" key={webhook_id.clone()}>
            { for webhook_requests.iter().map(|request| html! {
                <WebhookRequestDetails request={request.clone()} />
            }) }
        </div>
    }
}
