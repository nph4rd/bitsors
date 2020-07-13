extern crate bitsors;
#[macro_use]
extern crate serial_test;
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
#[serial]
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
#[serial]
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
#[serial]
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
#[serial]
async fn test_account_balance_unsuccessful() {
    let bitso = Bitso::default()
        .prefix("https://api-dev.bitso.com")
        .build();
    let result = bitso.get_account_balance().await;
    assert!(result.is_err());
    println!("{:?}", result);
}

/// Test successful request to get fees
#[tokio::test]
#[serial]
async fn test_fees_successful() {
    let bitso = Bitso::default()
        .prefix("https://api-dev.bitso.com")
        .client_credentials_manager(
            CLIENT_CREDENTIAL
            .lock()
            .unwrap()
            .clone()
        )
        .build();
    let result = bitso.get_fees().await;
    assert!(result.is_ok());
    println!("{:?}", result);
}

/// Test unsuccessful request to get fees
#[tokio::test]
#[serial]
async fn test_fees_unsuccessful() {
    let bitso = Bitso::default()
        .prefix("https://api-dev.bitso.com")
        .build();
    let result = bitso.get_fees().await;
    assert!(result.is_err());
    println!("{:?}", result);
}

/// Test successful request to get ledger
#[tokio::test]
#[serial]
async fn test_ledger_successful() {
    let bitso = Bitso::default()
        .prefix("https://api-dev.bitso.com")
        .client_credentials_manager(
            CLIENT_CREDENTIAL
            .lock()
            .unwrap()
            .clone()
        )
        .build();
    let result = bitso.get_ledger().await;
    assert!(result.is_ok());
    println!("{:?}", result);
}

/// Test unsuccessful request to get ledger
#[tokio::test]
#[serial]
async fn test_ledger_unsuccessful() {
    let bitso = Bitso::default()
        .prefix("https://api-dev.bitso.com")
        .build();
    let result = bitso.get_ledger().await;
    assert!(result.is_err());
    println!("{:?}", result);
}

/// Test successful request to get withdrawals
#[tokio::test]
#[serial]
async fn test_withdrawals_successful() {
    let bitso = Bitso::default()
        .prefix("https://api-dev.bitso.com")
        .client_credentials_manager(
            CLIENT_CREDENTIAL
            .lock()
            .unwrap()
            .clone()
        )
        .build();
    let result = bitso.get_withdrawals().await;
    assert!(result.is_ok());
    println!("{:?}", result);
}

/// Test unsuccessful request to get withdrawals
#[tokio::test]
#[serial]
async fn test_withdrawals_unsuccessful() {
    let bitso = Bitso::default()
        .prefix("https://api-dev.bitso.com")
        .build();
    let result = bitso.get_withdrawals().await;
    assert!(result.is_err());
    println!("{:?}", result);
}

/// Test successful request to get fundings
#[tokio::test]
#[serial]
async fn test_fundings_successful() {
    let bitso = Bitso::default()
        .prefix("https://api-dev.bitso.com")
        .client_credentials_manager(
            CLIENT_CREDENTIAL
            .lock()
            .unwrap()
            .clone()
        )
        .build();
    let result = bitso.get_fundings().await;
    assert!(result.is_ok());
    println!("{:?}", result);
}

/// Test unsuccessful request to get fundings
#[tokio::test]
#[serial]
async fn test_fundings_unsuccessful() {
    let bitso = Bitso::default()
        .prefix("https://api-dev.bitso.com")
        .build();
    let result = bitso.get_fundings().await;
    assert!(result.is_err());
    println!("{:?}", result);
}
