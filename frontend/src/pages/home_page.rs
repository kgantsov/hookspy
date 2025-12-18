use yew::prelude::*;

#[component]
pub fn HomePage() -> Html {
    html! {
        <div>
            <h1>{ "Welcome to Hookspy!" }</h1>
            <p>{ "This is a simple webhook management tool." }</p>
        </div>
    }
}
