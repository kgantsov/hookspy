use gloo_net::http::Request;
use yew::prelude::*;

use crate::components::webhook_request::WebhookRequest;
use crate::components::webhook_request::WebhookRequestDetails;

#[derive(Properties, PartialEq)]
pub struct WebhookRequestListProps {
    pub webhook_id: String,
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

    html! {
        <div class="requests-list" key={webhook_id.clone()}>
            { for webhook_requests.iter().map(|request| html! {
                <WebhookRequestDetails request={request.clone()} />
            }) }
        </div>
    }
}
