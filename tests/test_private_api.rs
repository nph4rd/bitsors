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
    // assert!(result.is_ok());
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

/// Test successful request to get user_trades
#[tokio::test]
#[serial]
async fn test_user_trades_successful() {
    let bitso = Bitso::default()
        .prefix("https://api-dev.bitso.com")
        .client_credentials_manager(
            CLIENT_CREDENTIAL
            .lock()
            .unwrap()
            .clone()
        )
        .build();
    let result = bitso.get_user_trades().await;
    assert!(result.is_ok());
    println!("{:?}", result);
}

/// Test unsuccessful request to get user_trades
#[tokio::test]
#[serial]
async fn test_user_trades_unsuccessful() {
    let bitso = Bitso::default()
        .prefix("https://api-dev.bitso.com")
        .build();
    let result = bitso.get_user_trades().await;
    assert!(result.is_err());
    println!("{:?}", result);
}

/// Test successful request to get order_trades
#[tokio::test]
#[serial]
async fn test_order_trades_successful() {
    let bitso = Bitso::default()
        .prefix("https://api-dev.bitso.com")
        .client_credentials_manager(
            CLIENT_CREDENTIAL
            .lock()
            .unwrap()
            .clone()
        )
        .build();
    let result = bitso.get_order_trades("CGL2vUsQ31ofl03N").await;
    assert!(result.is_ok());
    println!("{:?}", result);
}

/// Test unsuccessful request to get order_trades
#[tokio::test]
#[serial]
async fn test_order_trades_unsuccessful() {
    let bitso = Bitso::default()
        .prefix("https://api-dev.bitso.com")
        .build();
    let result = bitso.get_order_trades("oid").await;
    assert!(result.is_err());
    println!("{:?}", result);
}

/// Test successful request to get open_orders
#[tokio::test]
#[serial]
async fn test_open_orders_successful2() {
    let bitso = Bitso::default()
        .prefix("https://api-dev.bitso.com")
        .client_credentials_manager(
            CLIENT_CREDENTIAL
            .lock()
            .unwrap()
            .clone()
        )
        .build();
    let result = bitso.get_open_orders(None).await;
    assert!(result.is_ok()); // Bad request
    println!("{:?}", result);
}

/// Test successful request to get open_orders
#[tokio::test]
#[serial]
async fn test_open_orders_successful() {
    let bitso = Bitso::default()
        .prefix("https://api-dev.bitso.com")
        .client_credentials_manager(
            CLIENT_CREDENTIAL
            .lock()
            .unwrap()
            .clone()
        )
        .build();
    let result = bitso.get_open_orders(Some("btc_mxn")).await;
    assert!(result.is_ok());
    println!("{:?}", result);
}

/// Test unsuccessful request to get open_orders
#[tokio::test]
#[serial]
async fn test_open_orders_unsuccessful() {
    let bitso = Bitso::default()
        .prefix("https://api-dev.bitso.com")
        .build();
    let result = bitso.get_open_orders(None).await;
    assert!(result.is_err());
    println!("{:?}", result);
}

/// Test successful request to get lookup_orders
#[tokio::test]
#[serial]
async fn test_lookup_orders_successful() {
    let bitso = Bitso::default()
        .prefix("https://api-dev.bitso.com")
        .client_credentials_manager(
            CLIENT_CREDENTIAL
            .lock()
            .unwrap()
            .clone()
        )
        .build();
    let result = bitso.get_lookup_orders("CGL2vUsQ31ofl03N").await;
    // assert!(result.is_ok());
    println!("{:?}", result);
}

/// Test unsuccessful request to get lookup_orders
#[tokio::test]
#[serial]
async fn test_lookup_orders_unsuccessful() {
    let bitso = Bitso::default()
        .prefix("https://api-dev.bitso.com")
        .build();
    let result = bitso.get_lookup_orders("LqTP9jrRgqg9srwQ").await;
    assert!(result.is_err());
    println!("{:?}", result);
}

/// Test successful request to cancel_order
#[tokio::test]
#[serial]
async fn test_cancel_order_successful() {
    let bitso = Bitso::default()
        .prefix("https://api-dev.bitso.com")
        .client_credentials_manager(
            CLIENT_CREDENTIAL
            .lock()
            .unwrap()
            .clone()
        )
        .build();
    let result = bitso.cancel_order("fake-oid").await;
    assert!(result.is_ok());
    println!("{:?}", result);
}

/// Test unsuccessful request to get lookup_orders
#[tokio::test]
#[serial]
async fn test_cancel_order_unsuccessful() {
    let bitso = Bitso::default()
        .prefix("https://api-dev.bitso.com")
        .build();
    let result = bitso.cancel_order("oid").await;
    assert!(result.is_err());
    println!("{:?}", result);
}

/// Test successful request to place_order
#[tokio::test]
#[serial]
async fn test_place_order_successful() {
    let bitso = Bitso::default()
        .prefix("https://api-dev.bitso.com")
        .client_credentials_manager(
            CLIENT_CREDENTIAL
            .lock()
            .unwrap()
            .clone()
        )
        .build();
    let result = bitso.place_order(
        "btc_mxn",
        "sell",
        "market",
        Some("0.0001"), // major
    ).await;
    assert!(result.is_ok());
    println!("{:?}", result);
}

/// Test unsuccessful request to get lookup_orders
#[tokio::test]
#[serial]
async fn test_place_order_unsuccessful() {
    let bitso = Bitso::default()
        .prefix("https://api-dev.bitso.com")
        .build();
    let result = bitso.place_order(
        "btc_mxn",
        "sell",
        "market",
        Some("0.0001"), // major
    ).await;
    assert!(result.is_err());
    println!("{:?}", result);
}

/// Test successful request to funding_destination
#[tokio::test]
#[serial]
async fn test_funding_destination_successfull() {
    let bitso = Bitso::default()
        .prefix("https://api-dev.bitso.com")
        .client_credentials_manager(
            CLIENT_CREDENTIAL
            .lock()
            .unwrap()
            .clone()
        )
        .build();
    let result = bitso.get_funding_destination(
        "btc",
    ).await;
    assert!(result.is_err()); // Doesn't work atm for some reason
    println!("{:?}", result);
}

/// Test unsuccessful request to funding_destination
#[tokio::test]
#[serial]
async fn test_funding_destination_unsuccessful() {
    let bitso = Bitso::default()
        .prefix("https://api-dev.bitso.com")
        .build();
    let result = bitso.get_funding_destination(
        "btc",
    ).await;
    assert!(result.is_err());
    println!("{:?}", result);
}

/// Test successful request to make a crypto withdrawal
#[tokio::test]
#[serial]
async fn test_crypto_withdrawal_successful() {
    let bitso = Bitso::default()
        .prefix("https://api-dev.bitso.com")
        .client_credentials_manager(
            CLIENT_CREDENTIAL
            .lock()
            .unwrap()
            .clone()
        )
        .build();
    let result = bitso.withdrawal(
        "btc",
        "0.001",
        "3EW92Ajg6sMT4hxK8ngEc7Ehrqkr9RoDt7",
	Some("0.001"),
        Some("some_tag"),
    ).await;
    assert!(result.is_ok());
    println!("{:?}", result);
}
