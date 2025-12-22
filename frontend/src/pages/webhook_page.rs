use gloo_net::http::Request;
use web_sys::window;
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
                let resp = Request::get(&format!("/api/webhooks/{}", webhook_id))
                    .send()
                    .await;

                match resp {
                    Ok(resp) => {
                        if resp.status() == 401 {
                            if let Some(win) = window() {
                                let _ = win.location().set_href("/login");
                            }
                        } else {
                            let fetched_webhook: Result<Webhook, _> = resp.json().await;
                            match fetched_webhook {
                                Ok(fetched_webhook) => {
                                    webhook.set(Some(fetched_webhook));
                                }
                                Err(err) => web_sys::console::log_1(
                                    &format!("Error parsing webhook: {}", err).into(),
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
