use chrono::TimeZone;
use chrono_humanize::HumanTime;
use gloo_net::http::Request;
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

#[derive(Deserialize)]
struct IpApiResponse {
    status: String,
    country: Option<String>,
    #[serde(rename = "countryCode")]
    country_code: Option<String>,
    city: Option<String>,
}

fn country_code_to_flag(code: &str) -> String {
    let mut chars = code.chars();
    let (a, b) = match (chars.next(), chars.next()) {
        (Some(a), Some(b)) if a.is_ascii_alphabetic() && b.is_ascii_alphabetic() => (a, b),
        _ => return String::new(),
    };
    let base = 0x1F1E6u32 - b'A' as u32;
    let flag_a = char::from_u32(base + a.to_ascii_uppercase() as u32).unwrap_or(' ');
    let flag_b = char::from_u32(base + b.to_ascii_uppercase() as u32).unwrap_or(' ');
    format!("{}{}", flag_a, flag_b)
}

fn is_private_ip(ip: &str) -> bool {
    // Quick prefix checks — avoids pulling in any extra crate
    matches!(ip, "127.0.0.1" | "::1" | "localhost")
        || ip.starts_with("10.")
        || ip.starts_with("192.168.")
        || ip.starts_with("::ffff:127.")
        || {
            // 172.16.0.0/12
            if let Some(second) = ip.strip_prefix("172.") {
                second
                    .split('.')
                    .next()
                    .and_then(|n| n.parse::<u8>().ok())
                    .map(|n| (16..=31).contains(&n))
                    .unwrap_or(false)
            } else {
                false
            }
        }
}

async fn fetch_location(ip: &str) -> Option<String> {
    if is_private_ip(ip) {
        return None;
    }

    let url = format!(
        "https://ip-api.com/json/{}?fields=status,country,countryCode,city",
        ip
    );

    let resp = Request::get(&url).send().await.ok()?;
    let data: IpApiResponse = resp.json().await.ok()?;

    if data.status != "success" {
        return None;
    }

    let flag = data
        .country_code
        .as_deref()
        .map(country_code_to_flag)
        .unwrap_or_default();

    let city = data.city.unwrap_or_default();
    let country = data.country.unwrap_or_default();

    let location = match (city.is_empty(), country.is_empty()) {
        (false, false) => format!("{}, {} {}", city, country, flag),
        (true, false) => format!("{} {}", country, flag),
        (false, true) => city,
        (true, true) => return None,
    };

    Some(location.trim().to_string())
}

#[derive(Properties, PartialEq)]
pub struct WebhookRequestProps {
    pub request: WebhookRequest,
}

#[component]
pub fn WebhookRequestDetails(props: &WebhookRequestProps) -> Html {
    let expanded = use_state(|| false);
    // None  = not yet fetched
    // Some(None) = fetched but no result (private IP or lookup failed)
    // Some(Some(s)) = resolved location string
    let location: UseStateHandle<Option<Option<String>>> = use_state(|| None);

    let headers: Result<HashMap<String, String>, _> = serde_json::from_str(&props.request.headers);

    let formatted_body =
        if let Ok(json_value) = serde_json::from_str::<serde_json::Value>(&props.request.body) {
            serde_json::to_string_pretty(&json_value).unwrap_or_else(|_| props.request.body.clone())
        } else {
            props.request.body.clone()
        };

    let expanded_class = if *expanded { "expanded" } else { "" };

    let onclick = {
        let expanded = expanded.clone();
        let location = location.clone();
        let caller_ip = props.request.caller_ip.clone();

        Callback::from(move |_| {
            let will_expand = !*expanded;
            expanded.set(will_expand);

            // Trigger geolocation fetch the first time the card is expanded
            if will_expand && location.is_none() {
                if let Some(ref ip) = caller_ip {
                    let ip = ip.clone();
                    let location = location.clone();
                    wasm_bindgen_futures::spawn_local(async move {
                        let result = fetch_location(&ip).await;
                        location.set(Some(result));
                    });
                } else {
                    // No IP — mark as done so we don't retry
                    location.set(Some(None));
                }
            }
        })
    };

    // Flatten Option<Option<String>> -> Option<&str> for display
    let resolved_location: Option<&str> = location.as_ref().and_then(|inner| inner.as_deref());

    html! {
        <div class={format!("request-card {} {}", expanded_class, props.request.id.clone())}>
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
                                            {
                                                if let Some(ref ip) = props.request.caller_ip {
                                                    html! {
                                                        <span class="caller-info">
                                                            { "\u{1F4CD}" }
                                                            {
                                                                if let Some(loc) = resolved_location {
                                                                    html! { <span class="caller-location">{ loc }</span> }
                                                                } else {
                                                                    html! {}
                                                                }
                                                            }
                                                            <span class="caller-ip">{ ip.clone() }</span>
                                                        </span>
                                                    }
                                                } else {
                                                    html! {}
                                                }
                                            }
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
                    if props.request.caller_ip.is_some() || resolved_location.is_some() {
                        html! {
                            <div class="request-section">
                                <div class="section-title">{ "Caller" }</div>
                                <div class="key-value-list">
                                    {
                                        if let Some(ref ip) = props.request.caller_ip {
                                            html! {
                                                <div class="key-value-item">
                                                    <span class="key">{ "IP Address" }</span>
                                                    <span class="value">{ ip.clone() }</span>
                                                </div>
                                            }
                                        } else {
                                            html! {}
                                        }
                                    }
                                    {
                                        match location.as_ref() {
                                            None => html! {
                                                // Location not yet fetched — shown after first expand
                                                <></>
                                            },
                                            Some(None) => html! {
                                                // Lookup returned nothing (private IP or failed)
                                                <></>
                                            },
                                            Some(Some(loc)) => html! {
                                                <div class="key-value-item">
                                                    <span class="key">{ "Location" }</span>
                                                    <span class="value">{ loc.clone() }</span>
                                                </div>
                                            },
                                        }
                                    }
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
