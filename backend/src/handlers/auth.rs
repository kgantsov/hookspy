use axum::{
    extract::{Query, State},
    response::{IntoResponse, Redirect},
};
use oauth2::basic::BasicClient;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, RedirectUrl,
    Scope, TokenResponse, TokenUrl,
};
use tracing::{error, info};

use crate::{
    app::AppState,
    auth::{
        google::fetch_google_userinfo,
        jwt::{generate_app_jwt, set_auth_cookie},
        oauth2::{load_pkce_verifier, save_csrf_token, save_pkce_verifier, verify_csrf},
        user::{create_user, get_user},
    },
};

pub async fn login(State(state): State<AppState>) -> Redirect {
    let oauth_client = BasicClient::new(ClientId::new(state.config.oauth_client_id))
        .set_client_secret(ClientSecret::new(state.config.oauth_client_secret))
        .set_auth_uri(AuthUrl::new(state.config.oauth_auth_url).unwrap())
        .set_token_uri(TokenUrl::new(state.config.oauth_token_url).unwrap())
        .set_redirect_uri(RedirectUrl::new(state.config.oauth_redirect_url).unwrap());

    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    let (auth_url, csrf_token) = oauth_client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("openid".into()))
        .add_scope(Scope::new("email".into()))
        .add_scope(Scope::new("profile".into()))
        .set_pkce_challenge(pkce_challenge)
        .url();

    let state = auth_url
        .query_pairs()
        .find(|(k, _)| k == "state")
        .unwrap()
        .1
        .to_string();

    save_pkce_verifier(&state, pkce_verifier);
    save_csrf_token(&state, csrf_token.secret());

    Redirect::to(auth_url.as_str())
}

#[derive(serde::Deserialize, Debug)]
pub struct CallbackParams {
    code: String,
    state: String,
}

pub async fn callback(
    State(state): State<AppState>,
    Query(params): Query<CallbackParams>,
) -> impl IntoResponse {
    let pkce_verifier = match load_pkce_verifier(&params.state) {
        Some(verifier) => verifier,
        None => {
            error!("PKCE verifier error: Missing PKCE verifier");
            return Redirect::to("/").into_response();
        }
    };

    if !verify_csrf(&params.state, &params.state) {
        error!("CSRF verification failed");
        return Redirect::to("/").into_response();
    }

    let oauth_client = BasicClient::new(ClientId::new(state.config.oauth_client_id))
        .set_client_secret(ClientSecret::new(state.config.oauth_client_secret))
        .set_auth_uri(AuthUrl::new(state.config.oauth_auth_url).unwrap())
        .set_token_uri(TokenUrl::new(state.config.oauth_token_url).unwrap())
        .set_redirect_uri(RedirectUrl::new(state.config.oauth_redirect_url).unwrap());

    let http_client = reqwest::ClientBuilder::new()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .expect("Client should build");

    let token_result = oauth_client
        .exchange_code(AuthorizationCode::new(params.code))
        .set_pkce_verifier(pkce_verifier)
        .request_async(&http_client)
        .await;

    match token_result {
        Result::Ok(token) => {
            let access_token = token.access_token().secret();

            let userinfo = match fetch_google_userinfo(access_token).await {
                Result::Ok(user) => user,
                Result::Err(err) => {
                    error!("Failed to fetch Google userinfo: {:?}", err);
                    return Redirect::to("/").into_response();
                }
            };

            let db_guard = state.db.lock().await;

            let user = match get_user(db_guard.clone(), &userinfo.email).await {
                Result::Ok(user) => user,
                Result::Err(_err) => {
                    let user = create_user(
                        db_guard.clone(),
                        &userinfo.email,
                        userinfo.given_name.as_deref().unwrap_or(""),
                        userinfo.family_name.as_deref().unwrap_or(""),
                    )
                    .await;

                    match user {
                        Result::Ok(user) => {
                            info!("Created user with email: {}", &userinfo.email);
                            user
                        }
                        Result::Err(err) => {
                            error!("Failed to create user: {:?}", err);
                            return Redirect::to("/").into_response();
                        }
                    }
                }
            };

            let jwt = generate_app_jwt(state.config.jwt_secret.clone(), &user.id, &user.email);

            let headers = set_auth_cookie(&jwt);

            (headers, Redirect::to("/webhooks")).into_response()
        }
        Result::Err(err) => {
            error!("OAuth2 token exchange error: {:?}", err);
            Redirect::to("/").into_response()
        }
    }
}
