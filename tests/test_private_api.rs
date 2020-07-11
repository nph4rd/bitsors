extern crate bitsors;
#[macro_use]
extern crate lazy_static;

use std::sync::Mutex;
use bitsors::client::Bitso;
use bitsors::auth::BitsoCredentials;

lazy_static! {
    // Set api_key and api_secret in .env file or
    // export API_KEY="key"
    // export API_SECRET="secret"
    static ref CLIENT_CREDENTIAL: Mutex<BitsoCredentials> = Mutex::new(BitsoCredentials::default().build());
}

/// *** PRIVATE API *** ///


/// Test successful request to get account status
#[tokio::test]
async fn test_account_status_successful() {
    let bitso = Bitso::default()
        .prefix("https://api-dev.bitso.com")
        .client_credentials_manager(
            CLIENT_CREDENTIAL
            .lock()
            .unwrap()
            .clone()
        )
        .build();
    let result = bitso.get_account_status().await;
    assert!(result.is_ok());
    println!("{:?}", result);
}

/// Test unsuccessful request to get account status
#[tokio::test]
async fn test_account_status_unsuccessful() {
    let bitso = Bitso::default()
        .prefix("https://api-dev.bitso.com")
        .build();
    let result = bitso.get_account_status().await;
    assert!(result.is_err());
    println!("{:?}", result);
}

/// Test successful request to get account balance
#[tokio::test]
async fn test_account_balance_successful() {
    let bitso = Bitso::default()
        .prefix("https://api-dev.bitso.com")
        .client_credentials_manager(
            CLIENT_CREDENTIAL
            .lock()
            .unwrap()
            .clone()
        )
        .build();
    let result = bitso.get_account_balance().await;
    assert!(result.is_ok());
    println!("{:?}", result);
}

/// Test unsuccessful request to get account balance
#[tokio::test]
async fn test_account_balance_unsuccessful() {
    let bitso = Bitso::default()
        .prefix("https://api-dev.bitso.com")
        .build();
    let result = bitso.get_account_balance().await;
    assert!(result.is_err());
    println!("{:?}", result);
}
