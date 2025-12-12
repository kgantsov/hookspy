use serde::Deserialize;
use yew::prelude::*;

#[derive(Clone, PartialEq, Deserialize)]
pub struct Webhook {
    pub id: String,
    pub name: String,
    created_at: String,
}

#[derive(Properties, PartialEq)]
pub struct WebhookListProps {
    pub webhooks: Vec<Webhook>,
    pub on_click: Callback<Webhook>,
}

#[component]
pub fn WebhookList(WebhookListProps { webhooks, on_click }: &WebhookListProps) -> Html {
    let on_select = |webhook: &Webhook| {
        let on_click = on_click.clone();
        let webhook = webhook.clone();
        Callback::from(move |_: MouseEvent| on_click.emit(webhook.clone()))
    };

    html! {
        <div class="webhook-list">
            { for webhooks.iter().map(|webhook| {
                html! {
                    <div key={webhook.id.clone()} class="webhook-item" onclick={on_select(webhook)}>
                        <div class="webhook-info">
                            <div class="webhook-name">{webhook.name.clone()}</div>
                            <div class="webhook-id">{webhook.id.clone()}</div>
                        </div>
                        <div class="webhook-actions">
                            // <button
                            //     class="icon-btn danger"
                            // >
                            //     {"🗑️"}
                            // </button>
                        </div>
                    </div>
                }
            })}
        </div>
    }
}
