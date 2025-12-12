use chrono_humanize::HumanTime;
use serde::Deserialize;
use std::collections::HashMap;
use yew::prelude::*;

#[derive(Clone, PartialEq, Deserialize)]
pub struct WebhookRequest {
    pub id: String,
    pub webhook_id: String,
    pub method: String,
    pub headers: String,
    pub body: String,
    pub received_at: String,
}

#[derive(Properties, PartialEq)]
pub struct WebhookRequestProps {
    pub request: WebhookRequest,
}

#[component]
pub fn WebhookRequestDetails(props: &WebhookRequestProps) -> Html {
    let expanded = use_state(|| false);

    let headers: Result<HashMap<String, String>, _> = serde_json::from_str(&props.request.headers);

    let expanded_class = if *expanded { "expanded" } else { "" };

    let onclick = {
        let expanded = expanded.clone();
        Callback::from(move |_| expanded.set(!*expanded))
    };

    html! {
        <div
            class={format!("request-card {} {}", expanded_class, props.request.id.clone())}
            {onclick}
        >
            <div class="request-header">
                <div class="request-meta">
                    <span class="method-badge method-post">
                        { props.request.method.clone() }
                    </span>
                    <span class="request-time">
                        {
                            match chrono::DateTime::parse_from_rfc3339(&props.request.received_at) {
                                Ok(dt) => HumanTime::from(dt).to_string(),
                                Err(_) => props.request.received_at.clone(),
                            }
                        }
                    </span>
                </div>
                <span class="expand-icon">{ "▼" }</span>
            </div>
            <div class="request-body">
                <div class="request-section">
                    <div class="section-title">{ "Headers" }</div>
                    <div class="key-value-list">
                        {
                            match headers {
                                Ok(ref map) => {
                                    let mut items: Vec<_> = map.iter().collect();
                                    items.sort_by_key(|(key, _)| key.to_lowercase());
                                    items.into_iter().map(|(key, value)| {
                                        html! {
                                            <div class="key-value-item">
                                                <span class="key">{ key }</span>
                                                <span class="value">{ value }</span>
                                            </div>
                                        }
                                    }).collect::<Html>()
                                },
                                Err(_) => html! { <div class="key-value-item"><span class="key">{ "Invalid headers" }</span></div> },
                            }
                        }
                    </div>
                </div>
                <div class="request-section">
                    <div class="section-title">{ "Body" }</div>
                    <pre class="code-block">
                        { props.request.body.clone() }
                    </pre>
                </div>
            </div>
        </div>
    }
}
