use yew::prelude::*;

#[component]
pub fn LoginPage() -> Html {
    html! {
        <div>
            <h1>{ "Login" }</h1>
            <form>
                <label for="username">{ "Username" }</label>
                <input type="text" id="username" name="username" />
                <label for="password">{ "Password" }</label>
                <input type="password" id="password" name="password" />
                <button type="submit">{ "Login" }</button>
            </form>
        </div>
    }
}
