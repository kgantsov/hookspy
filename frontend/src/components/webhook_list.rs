use chrono::TimeZone;
use chrono_humanize::HumanTime;
use serde::Deserialize;
use yew::prelude::*;
use yew_router::prelude::Link;

use crate::hooks::use_clock_tick;
use crate::routes::Route;

#[derive(Clone, PartialEq, Deserialize)]
pub struct Webhook {
    pub id: String,
    pub name: String,
    pub url: String,
    pub created_at: String,
    pub has_unread: bool,
}

#[derive(Properties, PartialEq)]
pub struct WebhookListProps {
    pub webhooks: Vec<Webhook>,
    pub on_click: Callback<Webhook>,
    pub on_delete: Callback<Webhook>,
    pub selected_webhook_id: Option<String>,
}

#[component]
pub fn WebhookList(
    WebhookListProps {
        webhooks,
        on_click,
        on_delete,
        selected_webhook_id,
    }: &WebhookListProps,
) -> Html {
    // Tick every 30 s so relative timestamps ("created 5 minutes ago") stay
    // fresh without making any HTTP requests.
    let _tick = use_clock_tick(30_000);
    let on_select = |webhook: &Webhook| {
        let on_click = on_click.clone();
        let webhook = webhook.clone();
        Callback::from(move |_: MouseEvent| on_click.emit(webhook.clone()))
    };

    let on_delete_callback = |webhook: &Webhook| {
        let on_delete = on_delete.clone();
        let webhook = webhook.clone();
        Callback::from(move |_: MouseEvent| on_delete.emit(webhook.clone()))
    };

    html! {
        <div class="webhook-list">
            { for webhooks.iter().map(|webhook| {
                let is_active = selected_webhook_id.as_deref() == Some(webhook.id.as_str());
                let item_class = if is_active {
                    "webhook-item active"
                } else if webhook.has_unread {
                    "webhook-item unread"
                } else {
                    "webhook-item"
                };

                html! {
                    <Link<Route> to={Route::Webhook { webhook_id: webhook.id.clone() }}>
                        <div key={webhook.id.clone()} class={item_class} onclick={on_select(webhook)}>
                            <div class="webhook-info">
                                <div class="webhook-name">
                                    {webhook.name.clone()}
                                    {
                                        if webhook.has_unread && !is_active {
                                            html! { <span class="unread-dot" title="Unread requests"></span> }
                                        } else {
                                            html! {}
                                        }
                                    }
                                </div>
                                <div class="webhook-id">{webhook.id.clone()}</div>
                                <div class="webhook-created-at">
                                    {
                                        match chrono::DateTime::parse_from_rfc3339(&webhook.created_at) {
                                            Ok(dt) => {
                                                let offset_minutes = js_sys::Date::new_0().get_timezone_offset() as i32;
                                                let offset = chrono::FixedOffset::west_opt(offset_minutes * 60).unwrap();
                                                let local_dt = offset.from_utc_datetime(&dt.naive_utc());
                                                let tooltip = local_dt.format("%Y-%m-%d %H:%M:%S%.3f").to_string();
                                                let label = format!("Created {}", HumanTime::from(dt));
                                                html! { <span title={tooltip}>{ label }</span> }
                                            },
                                            Err(_) => html! { <span>{ format!("Created {}", webhook.created_at) }</span> },
                                        }
                                    }
                                </div>
                            </div>
                            <div class="webhook-actions">
                                <button
                                    class="icon-btn danger"
                                    onclick={on_delete_callback(webhook)}
                                >
                                    <svg width="14" height="14" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                        <polyline points="3 6 5 6 21 6"/>
                                        <path d="M19 6l-1 14a2 2 0 0 1-2 2H8a2 2 0 0 1-2-2L5 6"/>
                                        <path d="M10 11v6"/>
                                        <path d="M14 11v6"/>
                                        <path d="M9 6V4a1 1 0 0 1 1-1h4a1 1 0 0 1 1 1v2"/>
                                    </svg>
                                </button>
                            </div>
                        </div>
                    </Link<Route>>
                }
            })}
        </div>
    }
}
