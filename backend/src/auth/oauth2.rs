use lazy_static::lazy_static;
use oauth2::PkceCodeVerifier;
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    static ref PKCE_STORE: Mutex<HashMap<String, PkceCodeVerifier>> = Mutex::new(HashMap::new());
    static ref CSRF_STORE: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}

pub fn save_pkce_verifier(key: &str, verifier: PkceCodeVerifier) {
    PKCE_STORE.lock().unwrap().insert(key.to_string(), verifier);
}

pub fn load_pkce_verifier(key: &str) -> Option<PkceCodeVerifier> {
    PKCE_STORE.lock().unwrap().remove(key)
}

pub fn save_csrf_token(key: &str, csrf: &str) {
    CSRF_STORE
        .lock()
        .unwrap()
        .insert(key.to_string(), csrf.to_string());
}

pub fn verify_csrf(key: &str, csrf: &str) -> bool {
    CSRF_STORE
        .lock()
        .unwrap()
        .remove(key)
        .map_or(false, |v| v == csrf)
}
