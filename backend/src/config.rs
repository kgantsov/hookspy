#[derive(Clone)]
pub struct Config {
    pub oauth_client_id: String,
    pub oauth_client_secret: String,
    pub oauth_auth_url: String,
    pub oauth_token_url: String,
    pub oauth_redirect_url: String,
    pub jwt_secret: String,
}

// parse env variables and init Config
pub fn init_config() -> Config {
    let oauth_client_id = std::env::var("OAUTH_CLIENT_ID").expect("OAUTH_CLIENT_ID must be set");
    let oauth_client_secret =
        std::env::var("OAUTH_CLIENT_SECRET").expect("OAUTH_CLIENT_SECRET must be set");
    let oauth_auth_url = std::env::var("AUTH_URL").expect("AUTH_URL must be set");
    let oauth_token_url = std::env::var("TOKEN_URL").expect("TOKEN_URL must be set");
    let oauth_redirect_url = std::env::var("REDIRECT_URL").expect("REDIRECT_URL must be set");
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    Config {
        oauth_client_id,
        oauth_client_secret,
        oauth_auth_url,
        oauth_token_url,
        oauth_redirect_url,
        jwt_secret,
    }
}
