use chrono::TimeZone;
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
    pub caller_ip: Option<String>,
}

#[derive(Properties, PartialEq)]
pub struct WebhookRequestProps {
    pub request: WebhookRequest,
}

#[component]
pub fn WebhookRequestDetails(props: &WebhookRequestProps) -> Html {
    let expanded = use_state(|| false);

    let headers: Result<HashMap<String, String>, _> = serde_json::from_str(&props.request.headers);

    // Try to parse and pretty-print the body if it's JSON
    let formatted_body =
        if let Ok(json_value) = serde_json::from_str::<serde_json::Value>(&props.request.body) {
            serde_json::to_string_pretty(&json_value).unwrap_or_else(|_| props.request.body.clone())
        } else {
            props.request.body.clone()
        };

    let expanded_class = if *expanded { "expanded" } else { "" };

    let onclick = {
        let expanded = expanded.clone();
        Callback::from(move |_| expanded.set(!*expanded))
    };

    html! {
        <div
            class={format!("request-card {} {}", expanded_class, props.request.id.clone())}
        >
            <div class="request-header" {onclick}>
                <div class="request-meta">
                    <span class="method-badge method-post">
                        { props.request.method.clone() }
                    </span>
                    <span class="request-time">
                        {
                            match chrono::DateTime::parse_from_rfc3339(&props.request.received_at) {
                                Ok(dt) => {
                                    let offset_minutes = js_sys::Date::new_0().get_timezone_offset() as i32;
                                    let offset = chrono::FixedOffset::west_opt(offset_minutes * 60).unwrap();
                                    let local_dt = offset.from_utc_datetime(&dt.naive_utc());
                                    let precise = local_dt.format("%Y-%m-%d %H:%M:%S%.3f").to_string();
                                    let relative = HumanTime::from(dt).to_string();
                                    html! {
                                        <span class="request-time-block">
                                            <span class="request-time-precise">{ precise }</span>
                                            <span class="request-time-relative">{ relative }</span>
                                        </span>
                                    }
                                },
                                Err(_) => html! { <>{props.request.received_at.clone()}</> },
                            }
                        }
                    </span>
                </div>
                <span class="expand-icon">{ "▼" }</span>
            </div>
            <div class="request-body">
                {
                    if let Some(ref ip) = props.request.caller_ip {
                        html! {
                            <div class="request-section">
                                <div class="section-title">{ "Caller IP" }</div>
                                <div class="key-value-list">
                                    <div class="key-value-item">
                                        <span class="value">{ ip.clone() }</span>
                                    </div>
                                </div>
                            </div>
                        }
                    } else {
                        html! {}
                    }
                }
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
                                Err(_) => html! {
                                    <div class="key-value-item">
                                        <span class="key">{ "Invalid headers" }</span>
                                    </div>
                                },
                            }
                        }
                    </div>
                </div>
                <div class="request-section">
                    <div class="section-title">{ "Body" }</div>
                    <pre class="code-block">
                        { formatted_body }
                    </pre>
                </div>
            </div>
        </div>
    }
}
