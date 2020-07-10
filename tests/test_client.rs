extern crate bitsors;
extern crate reqwest;
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

/// Test successful request to get available books
#[tokio::test]
async fn test_available_books() {
    let bitso = Bitso::default()
        .build();
    let result = bitso.get_available_books().await;
    assert!(result.is_ok());
    println!("{:?}", result);
}

/// Test successful request to get ticker
#[tokio::test]
async fn test_ticker_successful() {
    let bitso = Bitso::default()
        .build();
    let result = bitso.get_ticker(Some("btc_mxn")).await;
    assert!(result.is_ok());
    println!("{:?}", result);
}

/// Test unsuccessful request to get ticker
#[tokio::test]
async fn test_ticker_unsuccessful() {
    let bitso = Bitso::default()
        .build();
    let result = bitso.get_ticker(Some("CREATEERROR")).await;
    assert!(!result.is_ok());
    println!("{:?}", result);
}

/// Test successful request to get order book
#[tokio::test]
async fn test_order_book_successful() {
    let bitso = Bitso::default()
        .build();
    let result = bitso.get_order_book(Some("btc_mxn")).await;
    assert!(result.is_ok());
    println!("{:?}", result);
}

/// Test unsuccessful request to get order book
#[tokio::test]
async fn test_order_book_unsuccessful() {
    let bitso = Bitso::default()
        .build();
    let result = bitso.get_order_book(Some("CREATEERROR")).await;
    assert!(!result.is_ok());
    println!("{:?}", result);
}

/// Test successful request to get trades
#[tokio::test]
async fn test_trades_successful() {
    let bitso = Bitso::default()
        .build();
    let result = bitso.get_trades(Some("btc_mxn")).await;
    assert!(result.is_ok());
    println!("{:?}", result);
}

/// Test unsuccessful request to get trades
#[tokio::test]
async fn test_trades_unsuccessful() {
    let bitso = Bitso::default()
        .build();
    let result = bitso.get_trades(Some("CREATEERROR")).await;
    assert!(!result.is_ok());
    println!("{:?}", result);
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
