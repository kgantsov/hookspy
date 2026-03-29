use chrono::TimeZone;
use chrono_humanize::HumanTime;
use serde::Deserialize;
use yew::prelude::*;
use yew_router::prelude::Link;

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
                                    {"🗑️"}
                                </button>
                            </div>
                        </div>
                    </Link<Route>>
                }
            })}
        </div>
    }
}
