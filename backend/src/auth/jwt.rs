use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use sha2::Sha256;
use time::{Duration, OffsetDateTime};

use serde::{Deserialize, Serialize};

use axum::http::{HeaderMap, HeaderValue};

use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};

type HmacSha256 = Hmac<Sha256>;

#[derive(Debug, Serialize, Deserialize)]
pub struct AppClaims {
    pub sub: String,
    pub email: String,
    pub exp: i64,
}

pub fn generate_app_jwt(secret: String, user_id: &str, email: &str) -> String {
    let key: HmacSha256 = HmacSha256::new_from_slice(secret.as_bytes()).unwrap();

    let exp = (OffsetDateTime::now_utc() + Duration::hours(24)).unix_timestamp();

    let claims = AppClaims {
        sub: user_id.to_string(),
        email: email.to_string(),
        exp,
    };

    claims.sign_with_key(&key).unwrap()
}

pub fn verify_jwt(secret: String, token: &str) -> anyhow::Result<AppClaims> {
    use jwt::VerifyWithKey;

    let key: HmacSha256 = HmacSha256::new_from_slice(secret.as_bytes())?;

    let claims: AppClaims = token.verify_with_key(&key)?;
    Ok(claims)
}

pub fn set_auth_cookie(jwt: &str) -> HeaderMap {
    let mut headers = HeaderMap::new();

    headers.insert(
        axum::http::header::SET_COOKIE,
        HeaderValue::from_str(&format!(
            "auth_token={}; HttpOnly; Path=/; SameSite=Lax; Secure",
            jwt
        ))
        .unwrap(),
    );

    headers
}

pub struct AuthUser(pub AppClaims);

#[async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let cookies = parts
            .headers
            .get(axum::http::header::COOKIE)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("");

        let token = cookies
            .split(';')
            .find_map(|c| {
                let c = c.trim();
                c.strip_prefix("auth_token=")
            })
            .ok_or((StatusCode::UNAUTHORIZED, "Missing auth"))?;

        let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "your_secret_here".to_string());

        let claims =
            verify_jwt(secret, token).map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid token"))?;

        let now = time::OffsetDateTime::now_utc().unix_timestamp();
        if claims.exp < now {
            return Err((StatusCode::UNAUTHORIZED, "Expired token"));
        }

        Ok(AuthUser(claims))
    }
}
