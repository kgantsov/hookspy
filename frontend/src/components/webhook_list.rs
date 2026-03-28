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
                } else {
                    "webhook-item"
                };

                html! {
                    <Link<Route> to={Route::Webhook { webhook_id: webhook.id.clone() }}>
                        <div key={webhook.id.clone()} class={item_class} onclick={on_select(webhook)}>
                            <div class="webhook-info">
                                <div class="webhook-name">{webhook.name.clone()}</div>
                                <div class="webhook-id">{webhook.id.clone()}</div>
                                <div class="webhook-created-at">
                                    {
                                        match chrono::DateTime::parse_from_rfc3339(&webhook.created_at) {
                                            Ok(dt) => format!("Created {}", HumanTime::from(dt)),
                                            Err(_) => format!("Created {}", webhook.created_at),
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
