use yew::prelude::*;

#[component]
pub fn HomePage() -> Html {
    let login = Callback::from(|_: MouseEvent| {
        web_sys::window()
            .unwrap()
            .location()
            .set_href("/api/auth/login")
            .unwrap();
    });

    html! {
           <div class="landing">

               <nav class="landing-nav">
                   <div class="landing-nav-inner">
                       <div class="landing-logo">
                           <div class="landing-logo-icon">
                               <svg width="20" height="20" viewBox="0 0 20 20" fill="none" xmlns="http://www.w3.org/2000/svg">
                                   <circle cx="7" cy="5" r="2.5" stroke="white" stroke-width="1.75" fill="none"/>
                                   <circle cx="7" cy="5" r="1" fill="white"/>
                                   <path d="M7 7.5 L7 12 Q7 17 12 17
    Q17 17 17 12 Q17 9.5 14.5 9.5"
                                       stroke="white" stroke-width="2"
                                       stroke-linecap="round" stroke-linejoin="round" fill="none"/>
                               </svg>
                           </div>
                           <span>{"HookSpy"}</span>
                       </div>
                       <button onclick={login.clone()}
                           class="landing-cta-btn"
                           style="padding: 0.5rem 1.25rem; font-size: 0.875rem;">
                           {"Sign in"}
                       </button>
                   </div>
               </nav>

               <div class="landing-hero-bg" />

               <section class="landing-hero">

                   <div class="landing-hero-content">
                       <div class="landing-badge">{"Real-time webhook inspector"}</div>

                       <h1 class="landing-title">
                           {"Inspect webhooks"}
                           <br/>
                           {"in real-time."}
                       </h1>

                       <p class="landing-subtitle">
                           {"Create instant endpoints, point any service at them, and \
                             watch requests arrive live. Debug integrations without \
                             any server setup."}
                       </p>

                       <button onclick={login} class="landing-cta-btn">
                           <svg class="landing-cta-google-icon"
                               viewBox="0 0 48 48"
                               xmlns="http://www.w3.org/2000/svg">
                               <path fill="#EA4335"
                                   d="M24 9.5c3.54 0 6.7 1.22 9.19 3.61l6.85-6.85C35.9 2.38 \
                                      30.47 0 24 0 14.62 0 6.51 5.38 2.56 13.22l7.98 \
                                      6.19C12.43 13.09 17.74 9.5 24 9.5z"/>
                               <path fill="#4285F4"
                                   d="M46.98 24.55c0-1.57-.15-3.09-.38-4.55H24v9.02h12.94 \
                                   c-.58 3.02-2.33 5.58-4.94
 7.3l7.56 5.88c4.42-4.08 \
                                      6.92-10.1 6.92-17.65z"/>
                               <path fill="#FBBC05"
                                   d="M10.53 28.59c-.48-1.45-.76-2.99-.76-4.59s.27-3.14 \
                                      .76-4.59l-7.98-6.19C.92 16.46 0 20.12 0 24s.92 \
                                      7.54 2.56 10.78l7.97-6.19z"/>
                               <path fill="#34A853"
                                   d="M24 48c6.48 0 11.93-2.13 15.9-5.81l-7.56-5.88 \
                                      c-2.1 1.41-4.78 2.24-8.34 2.24-6.26 0-11.57-3.59 \
                                      -13.47-8.83l-7.98 6.19C6.51 42.62 14.62 48 24 48z"/>
                           </svg>
                           {"Get started free"}
                       </button>
                   </div>

                   <div class="landing-mockup">
                       <div class="landing-mockup-window">

                           <div class="landing-mockup-titlebar">
                               <div class="landing-mockup-dot red"   />
                               <div class="landing-mockup-dot yellow" />
                               <div class="landing-mockup-dot green"  />
                               <div class="landing-mockup-url">
                                   {"hookspy.dev · /webhooks/a3f9b2c1"}
                               </div>
                               <div class="landing-mock-live-badge">
                                   <div class="landing-mock-live-dot" />
                                   {"LIVE"}
                               </div>
                           </div>

                           <div class="landing-mockup-body">

                               <div class="landing-mock-request is-new">
                                   <div class="landing-mock-req-header">
                                       <span class="landing-mock-method post">{"POST"}</span>
                                       <span class="landing-mock-path">{"/webhooks/a3f9b2c1"}</span>
                                       <span class="landing-mock-time">{"just now"}</span>
                                   </div>
                                   <div
    class="landing-mock-req-body">
                                       <span class="jb">{"{ "}</span>
                                       <span class="jk">{r#""event""#}</span>
                                       <span class="jb">{": "}</span>
                                       <span class="js">{r#""payment.completed""#}</span>
                                       <span class="jb">{", "}</span>
                                       <span class="jk">{r#""amount""#}</span>
                                       <span class="jb">{": "}</span>
                                       <span class="jn">{"4990"}</span>
                                       <span class="jb">{", "}</span>
                                       <span class="jk">{r#""currency""#}</span>
                                       <span class="jb">{": "}</span>
                                       <span class="js">{r#""usd""#}</span>
                                       <span class="jb">{" }"}</span>
                                   </div>
                               </div>

                               <div class="landing-mock-request is-old">
                                   <div class="landing-mock-req-header">
                                       <span class="landing-mock-method post">{"POST"}</span>
                                       <span class="landing-mock-path">{"/webhooks/a3f9b2c1"}</span>
                                       <span class="landing-mock-time">{"14s ago"}</span>
                                   </div>
                                   <div class="landing-mock-req-body">
                                       <span class="jb">{"{ "}</span>
                                       <span class="jk">{r#""event""#}</span>
                                       <span class="jb">{": "}</span>
                                       <span class="js">{r#""user.signup""#}</span>
                                       <span class="jb">{", "}</span>
                                       <span class="jk">{r#""email""#}</span>
                                       <span class="jb">{": "}</span>
                                       <span class="js">{r#""alex@acme.io""#}</span>
                                       <span class="jb">{" }"}</span>
                                   </div>
                               </div>

                               <div class="landing-mock-request is-old">
                                   <div class="landing-mock-req-header">
                                       <span class="landing-mock-method get">{"GET"}</span>
                                       <span class="landing-mock-path">{"/webhooks/a3f9b2c1"}</span>
                                       <span class="landing-mock-time">{"1m ago"}</span>
                                   </div>
                                   <div class="landing-mock-req-body">
                                       <span class="jb">{"{ "}</span>
                                       <span class="jk">{r#""status""#}</span>
                                       <span class="jb">{": "}</span>
                                       <span class="js">{r#""ok""#}</span>
                                       <span class="jb">{", "}</span>
                                       <span class="jk">{r#""ping""#}</span>
                                       <span class="jb">{": "}</span>
                                       <span class="jn">{"true"}</span>
                                       <span class="jb">{" }"}</span>
                                   </div>
                               </div>

                           </div>
                       </div>
                   </div>

               </section>

               <hr class="landing-divider" />

               <section class="landing-features">
                   <p class="landing-features-eyebrow">{"Why HookSpy"}</p>
                   <h2 class="landing-features-title">
                       {"Everything you need to debug webhooks"}
                   </h2>
                   <p class="landing-features-sub">
                       {"No proxies, no CLI tools, no extra configuration. Just a URL and a browser."}
                   </p>

                   <div class="landing-features-grid">

                       <div class="landing-feature-card">
                           <div class="landing-feature-icon">
                               <svg width="22" height="22" viewBox="0 0 24 24" fill="none"
                                   stroke="currentColor" stroke-width="2"
                                   stroke-linecap="round" stroke-linejoin="round">
                                   <polygon points="13 2 3 14 12 14 11 22 21 10 12 10 13 2"/>
                               </svg>
                           </div>
                           <div class="landing-feature-title">{"Instant endpoints"}</div>
                           <p class="landing-feature-desc">
                               {"Generate a unique webhook URL in seconds. No DNS setup, no \
                                 server — share it immediately and start capturing requests \
                                 from any service."}
                           </p>
                       </div>

                       <div class="landing-feature-card">
                           <div class="landing-feature-icon">
                               <svg width="22" height="22" viewBox="0 0 24 24" fill="none"
                                   stroke="currentColor" stroke-width="2"
                                   stroke-linecap="round" stroke-linejoin="round">
                                   <path d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z"/>
                                   <circle cx="12" cy="12" r="3"/>
                               </svg>
                           </div>
                           <div class="landing-feature-title">{"Live inspection"}</div>
                           <p class="landing-feature-desc">
                               {"Watch requests arrive in real-time over WebSocket. Full \
                                 headers, body, and query params — everything visible the \
                                 moment it lands."}
                           </p>
                       </div>

                       <div class="landing-feature-card">
                           <div class="landing-feature-icon">
                               <svg width="22" height="22" viewBox="0 0 24 24" fill="none"
                                   stroke="currentColor" stroke-width="2"
                                   stroke-linecap="round" stroke-linejoin="round">
                                   <circle cx="11" cy="11" r="8"/>
                                   <line x1="21" y1="21" x2="16.65" y2="16.65"/>
                               </svg>
                           </div>
                           <div class="landing-feature-title">{"Powerful search"}</div>
                           <p class="landing-feature-desc">
                               {"Filter across all captured requests by content, method, or \
                                 timestamp. Find exactly what you need without endless \
                                 scrolling."}
                           </p>
                       </div>

                       <div class="landing-feature-card">
                           <div class="landing-feature-icon">
                               <svg width="22" height="22" viewBox="0 0 24 24" fill="none"
                                   stroke="currentColor" stroke-width="2"
                                   stroke-linecap="round" stroke-linejoin="round">
                                   <path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"/>
                               </svg>
                           </div>
                           <div class="landing-feature-title">{"Secure by default"}</div>
                           <p class="landing-feature-desc">
                               {"Sign in with Google OAuth2. Your endpoints and request \
                                 history are private — only you can access them."}
                           </p>
                       </div>

                       <div class="landing-feature-card">
                           <div class="landing-feature-icon">
                               <svg width="22" height="22" viewBox="0 0 24 24" fill="none"
                                   stroke="currentColor" stroke-width="2"
                                   stroke-linecap="round" stroke-linejoin="round">
                                   <rect x="2" y="3" width="20" height="14" rx="2" ry="2"/>
                                   <line x1="8" y1="21" x2="16" y2="21"/>
                                   <line x1="12" y1="17" x2="12" y2="21"/>
                               </svg>
                           </div>
                           <div class="landing-feature-title">{"Multiple webhooks"}</div>
                           <p class="landing-feature-desc">
                               {"Create as many endpoints as you need. Manage different \
                                 integrations in parallel from one clean dashboard."}
                           </p>
                       </div>

                       <div class="landing-feature-card">
                           <div class="landing-feature-icon">
                               <svg width="22" height="22" viewBox="0 0 24 24" fill="none"
                                   stroke="currentColor" stroke-width="2"
                                   stroke-linecap="round" stroke-linejoin="round">

                                   <polyline points="16 18 22 12 16 6"/>
                                   <polyline points="8 6 2 12 8 18"/>
                               </svg>
                           </div>
                           <div class="landing-feature-title">{"Zero setup"}</div>
                           <p class="landing-feature-desc">
                               {"Built with Rust and WebAssembly — blazing fast with no \
                                 runtime dependencies. Open a tab and you are ready to go."}
                           </p>
                       </div>

                   </div>
               </section>

               <footer class="landing-footer">
                   {"© 2025 HookSpy · Built for developers"}
               </footer>

           </div>
       }
}
