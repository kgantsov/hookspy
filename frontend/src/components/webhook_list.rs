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
}

#[component]
pub fn WebhookList(
    WebhookListProps {
        webhooks,
        on_click,
        on_delete,
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
                html! {
                    <Link<Route> to={Route::Webhook { webhook_id: webhook.id.clone() }}>
                        <div key={webhook.id.clone()} class="webhook-item" onclick={on_select(webhook)}>
                            <div class="webhook-info">
                                <div class="webhook-name">{webhook.name.clone()}</div>
                                <div class="webhook-id">{webhook.id.clone()}</div>
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
