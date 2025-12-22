use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::login_page::LoginPage;
use crate::pages::webhook_page::WebhookPage;
use crate::pages::webhooks_layout::WebhooksLayout;

#[component]
pub fn EmptyState() -> Html {
    html! {
        <></>
    }
}

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,

    #[at("/login")]
    Login,

    #[at("/webhooks")]
    Webhooks,

    #[at("/webhooks/:webhook_id")]
    Webhook { webhook_id: String },

    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Home | Route::Login => html! { <LoginPage /> },
        Route::Webhooks => html! {
            <WebhooksLayout>
                <EmptyState />
            </WebhooksLayout>
        },
        Route::Webhook { webhook_id } => html! {
            <WebhooksLayout>
                <WebhookPage webhook_id={webhook_id} />
            </WebhooksLayout>
        },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}
