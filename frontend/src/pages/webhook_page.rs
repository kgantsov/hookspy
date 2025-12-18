use gloo_net::http::Request;
use yew::prelude::*;

use crate::components::webhook_details::WebhookDetails;
use crate::components::webhook_list::Webhook;

#[derive(Properties, PartialEq)]
pub struct WebhookPageProps {
    pub webhook_id: String,
}

#[component]
pub fn WebhookPage(WebhookPageProps { webhook_id }: &WebhookPageProps) -> Html {
    let webhook = use_state(|| None::<Webhook>);
    {
        let webhook = webhook.clone();
        let webhook_id = webhook_id.clone();
        use_effect_with(webhook_id.clone(), move |_| {
            let webhook = webhook.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_webhook: Webhook =
                    Request::get(&format!("/api/webhooks/{}", webhook_id))
                        .send()
                        .await
                        .unwrap()
                        .json()
                        .await
                        .unwrap();
                webhook.set(Some(fetched_webhook));
            });
            || ()
        });
    }

    html! {
        <>
            {
                if let Some(webhook) = webhook.as_ref() {
                    html! { <WebhookDetails webhook={webhook.clone()} /> }
                } else {
                    html! {}
                }
            }
        </>
    }
}
