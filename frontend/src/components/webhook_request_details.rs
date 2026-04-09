use chrono::TimeZone;
use chrono_humanize::HumanTime;
use serde::Deserialize;
use std::collections::HashMap;
use yew::prelude::*;

use crate::hooks::use_clock_tick;

use crate::components::tooltip::Tooltip;

#[derive(Clone, PartialEq, Deserialize)]
pub struct WebhookRequest {
    pub id: String,
    pub webhook_id: String,
    pub method: String,
    pub headers: String,
    pub body: String,
    pub received_at: String,
    pub caller_ip: Option<String>,
    pub duration_us: Option<u64>,
}

#[derive(Properties, PartialEq)]
pub struct WebhookRequestProps {
    pub request: WebhookRequest,
    pub search_query: String,
}

/// Splits `text` around case-insensitive occurrences of `query` and returns
/// an `Html` fragment where each match is wrapped in `<mark class="search-highlight">`.
fn highlight_text(text: &str, query: &str) -> Html {
    if query.is_empty() {
        return html! { {text} };
    }

    // Build char-index arrays so we handle multi-byte Unicode safely.
    let text_chars: Vec<(usize, char)> = text.char_indices().collect();
    let query_lower: Vec<char> = query.chars().flat_map(|c| c.to_lowercase()).collect();
    let q_len = query_lower.len(); // length in chars

    if q_len == 0 || text_chars.is_empty() {
        return html! { {text} };
    }

    let t_len = text_chars.len();
    let mut parts: Vec<Html> = Vec::new();
    let mut i = 0usize; // char index into text_chars
    let mut last_byte = 0usize;

    while i + q_len <= t_len {
        let is_match = text_chars[i..i + q_len]
            .iter()
            .zip(query_lower.iter())
            .all(|((_, tc), qc)| tc.to_lowercase().next().unwrap_or(*tc) == *qc);

        if is_match {
            let start_byte = text_chars[i].0;
            let end_byte = if i + q_len < t_len {
                text_chars[i + q_len].0
            } else {
                text.len()
            };

            if start_byte > last_byte {
                let before = text[last_byte..start_byte].to_owned();
                parts.push(html! { {before} });
            }
            let matched = text[start_byte..end_byte].to_owned();
            parts.push(html! {
                <mark class="search-highlight">{ matched }</mark>
            });
            last_byte = end_byte;
            i += q_len;
        } else {
            i += 1;
        }
    }

    if last_byte < text.len() {
        let tail = text[last_byte..].to_owned();
        parts.push(html! { {tail} });
    }

    html! { <>{ for parts.into_iter() }</> }
}

#[component]
pub fn WebhookRequestDetails(props: &WebhookRequestProps) -> Html {
    let expanded = use_state(|| false);
    // Tick every 30 s so relative timestamps ("7 minutes ago") stay
    // fresh without making any HTTP requests.
    let _tick = use_clock_tick(30_000);

    let headers: Result<HashMap<String, String>, _> = serde_json::from_str(&props.request.headers);

    // Try to parse and pretty-print the body if it's JSON
    let formatted_body =
        if let Ok(json_value) = serde_json::from_str::<serde_json::Value>(&props.request.body) {
            serde_json::to_string_pretty(&json_value).unwrap_or_else(|_| props.request.body.clone())
        } else {
            props.request.body.clone()
        };

    let body_size = props.request.body.len();
    let size_label = if body_size < 1_024 {
        format!("{} B", body_size)
    } else if body_size < 1_048_576 {
        format!("{:.2} KB", body_size as f64 / 1_024.0)
    } else {
        format!("{:.2} MB", body_size as f64 / 1_048_576.0)
    };

    let expanded_class = if *expanded { "expanded" } else { "" };

    let onclick = {
        let expanded = expanded.clone();
        Callback::from(move |_| expanded.set(!*expanded))
    };

    let query = props.search_query.clone();

    html! {
        <div
            class={format!("request-card {} {}", expanded_class, props.request.id.clone())}
        >

            <div class="request-header">
                <div class="request-meta" {onclick}>
                    <span class="method-badge method-post">
                        { highlight_text(&props.request.method, &query) }
                    </span>
                    {
                        if let Some(duration) = props.request.duration_us {
                            let label = if duration < 1_000 {
                                format!("{} µs", duration)
                            } else if duration < 1_000_000 {
                                format!("{:.2} ms", duration as f64 / 1_000.0)
                            } else {
                                format!("{:.2} s", duration as f64 / 1_000_000.0)
                            };
                            html! {
                                <Tooltip text="Time taken to process this request">
                                    <span class="duration-badge">
                                        { label }
                                    </span>
                                </Tooltip>
                            }
                        } else {
                            html! {}
                        }
                    }
                    <Tooltip text="Payload size of the request body">
                        <span class="size-badge">
                            { size_label.clone() }
                        </span>
                    </Tooltip>
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
                                        <span class="value">{ highlight_text(ip, &query) }</span>
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
                                                <span class="key">{ highlight_text(key, &query) }</span>
                                                <span class="value">{ highlight_text(value, &query) }</span>
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
                        { highlight_text(&formatted_body, &query) }
                    </pre>
                </div>
            </div>
        </div>
    }
}
