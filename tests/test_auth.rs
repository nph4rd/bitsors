extern crate bitsors;
extern crate dotenv;

use std::env;
use dotenv::dotenv;
use bitsors::auth::BitsoCredentials;

/// Test credential set-up
#[test]
fn test_credentials_setup() {
    dotenv().ok();
    let api_key = env::var("API_KEY")
        .unwrap_or_default();
    let api_secret = env::var("API_SECRET")
        .unwrap_or_default();
    let auth = BitsoCredentials::default()
        .build();
    assert_eq!(api_key, auth.get_key());
    assert_eq!(api_secret, auth.get_secret());
}

/// Test API_KEY
#[test]
fn test_set_key() {
    let auth = BitsoCredentials::default()
        .api_key("KEY")
        .build();
    assert_eq!("KEY", auth.get_key());
}

/// Test API_SECRET
#[test]
fn test_set_secret() {
    let auth = BitsoCredentials::default()
        .api_secret("SECRET")
        .build();
    assert_eq!("SECRET", auth.get_secret());
}
