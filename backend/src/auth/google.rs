use anyhow::Ok;
use reqwest::Client;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize)]
pub struct GoogleUserInfo {
    pub sub: String,
    pub email: String,
    pub email_verified: bool,
    pub given_name: Option<String>,
    pub family_name: Option<String>,
    pub name: Option<String>,
    pub picture: Option<String>,
}

pub async fn fetch_google_userinfo(access_token: &str) -> anyhow::Result<GoogleUserInfo> {
    let client = Client::new();

    let resp = client
        .get("https://openidconnect.googleapis.com/v1/userinfo")
        .bearer_auth(access_token)
        .send()
        .await?
        .error_for_status()?;

    let user: GoogleUserInfo = serde_json::from_slice(&resp.bytes().await?)?;

    Ok(user)
}
