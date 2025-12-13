use gloo_net::http::Request;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct CreateWebhookModalProps {
    pub is_open: bool,
    pub on_close: Callback<()>,
}

#[component]
pub fn CreateWebhookModal(
    CreateWebhookModalProps { is_open, on_close }: &CreateWebhookModalProps,
) -> Html {
    let name = use_state(|| String::new());

    let on_input = {
        let name = name.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            name.set(input.value());
        })
    };

    let on_close_callback = {
        let on_close = on_close.clone();
        Callback::from(move |_: MouseEvent| on_close.emit(()))
    };

    let on_close_submit = on_close.clone();

    let on_submit_callback = {
        let name = name.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            let webhook_name = (*name).clone();
            let on_close = on_close_submit.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let req = Request::post("/api/webhooks")
                    .header("Content-Type", "application/json")
                    .body(
                        serde_json::json!({
                            "name": webhook_name,
                        })
                        .to_string(),
                    );

                match req {
                    Ok(req) => {
                        let res = req.send().await;

                        match res {
                            Ok(_) => on_close.emit(()),
                            Err(err) => {
                                web_sys::console::error_1(&err.to_string().into());
                            }
                        }
                    }
                    Err(err) => {
                        web_sys::console::error_1(&err.to_string().into());
                    }
                }
            });
        })
    };

    if !is_open {
        return html! {};
    }

    html! {
        <div class="modal active" id="createModal">
            <div class="modal-content">
                <h2 class="modal-header">{ "Create New Webhook" }</h2>
                <form onsubmit={on_submit_callback}>
                    <div class="form-group">
                        <label class="form-label">{ "Webhook Name" }</label>
                        <input
                            type="text"
                            class="form-input"
                            placeholder="e.g., Payment Gateway"
                            required=true
                            value={(*name).clone()}
                            oninput={on_input}
                        />
                    </div>
                    <div class="modal-actions">
                        <button
                            type="button"
                            class="btn btn-danger btn-sm"
                            onclick={on_close_callback}
                        >
                            { "Cancel" }
                        </button>
                        <button type="submit" class="btn btn-primary btn-sm">
                            { "Create Webhook" }
                        </button>
                    </div>
                </form>
            </div>
        </div>
    }
}
