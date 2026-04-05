use yew::prelude::*;

#[component]
pub fn LoginPage() -> Html {
    let login = Callback::from(|_: MouseEvent| {
        web_sys::window()
            .unwrap()
            .location()
            .set_href("/api/auth/login")
            .unwrap();
    });

    html! {
        <div class="auth-page">
            <div class="auth-container">
                <div class="auth-card">
                    <div class="auth-logo">
                        <div class="auth-logo-icon">
                            <svg width="26" height="26" viewBox="0 0 20 20" fill="none" xmlns="http://www.w3.org/2000/svg">
                                <circle cx="7" cy="5" r="2.5" stroke="white" stroke-width="1.75" fill="none"/>
                                <circle cx="7" cy="5" r="1" fill="white"/>
                                <path d="M7 7.5 L7 12 Q7 17 12 17 Q17 17 17 12 Q17 9.5 14.5 9.5" stroke="white" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" fill="none"/>
                            </svg>
                        </div>
                        <span>{ "HookSpy" }</span>
                    </div>

                    <h1 class="auth-title">{ "Sign in to HookSpy" }</h1>
                    <p class="auth-subtitle">
                        { "Secure webhook inspection with Google authentication" }
                    </p>

                    <a onclick={login} class="google-btn">
                        <svg
                            class="google-icon"
                            viewBox="0 0 48 48"
                            xmlns="http://www.w3.org/2000/svg"
                        >
                            <path
                                fill="#EA4335"
                                d="M24 9.5c3.54 0 6.7 1.22 9.19 3.61l6.85-6.85C35.9 2.38 30.47 0 24 0 14.62 0 6.51 5.38 2.56 13.22l7.98 6.19C12.43 13.09 17.74 9.5 24 9.5z"
                            />
                            <path
                                fill="#4285F4"
                                d="M46.98 24.55c0-1.57-.15-3.09-.38-4.55H24v9.02h12.94c-.58 3.02-2.33 5.58-4.94 7.3l7.56 5.88c4.42-4.08 6.92-10.1 6.92-17.65z"
                            />
                            <path
                                fill="#FBBC05"
                                d="M10.53 28.59c-.48-1.45-.76-2.99-.76-4.59s.27-3.14.76-4.59l-7.98-6.19C.92 16.46 0 20.12 0 24s.92 7.54 2.56 10.78l7.97-6.19z"
                            />
                            <path
                                fill="#34A853"
                                d="M24 48c6.48 0 11.93-2.13 15.9-5.81l-7.56-5.88c-2.1 1.41-4.78 2.24-8.34 2.24-6.26 0-11.57-3.59-13.47-8.83l-7.98 6.19C6.51 42.62 14.62 48 24 48z"
                            />
                        </svg>
                        { "Continue with Google" }
                    </a>

                    <div class="auth-footer">
                        { "By continuing, you agree to our" }
                        <a href="/terms">{ "Terms" }</a>{ " and" }
                        <a href="/privacy">{ "Privacy Policy" }</a>
                    </div>
                </div>
            </div>
            </div>
    }
}
