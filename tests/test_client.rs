extern crate bitsors;
extern crate reqwest;

use reqwest::Method;
use bitsors::auth::BitsoCredentials;
use bitsors::client::Bitso;

/// Test successful request to get available books
#[tokio::test]
async fn test_available_books() {
    let bitso = Bitso::default()
        .build();
    let result = bitso.get_available_books().await;
    println!("{:?}", result);
}

/// Test successful request to get ticker
#[tokio::test]
async fn test_ticker_successfull() {
    let bitso = Bitso::default()
        .build();
    let result = bitso.get_ticker(Some("btc_mxn")).await;
    println!("{:?}", result);
}

/// Test unsuccessful request to get ticker
#[tokio::test]
async fn test_ticker_unsuccessfull() {
    let bitso = Bitso::default()
        .build();
    let result = bitso.get_ticker(Some("CREATEERROR")).await;
    println!("{:?}", result);
}

/// Test successful request to get order book
#[tokio::test]
async fn test_order_book_successfull() {
    let bitso = Bitso::default()
        .build();
    let result = bitso.get_order_book(Some("btc_mxn")).await;
    println!("{:?}", result);
}

/// Test unsuccessful request to get order book
#[tokio::test]
async fn test_order_book_unsuccessfull() {
    let bitso = Bitso::default()
        .build();
    let result = bitso.get_order_book(Some("CREATEERROR")).await;
    println!("{:?}", result);
}

/// Test successful request to get trades
#[tokio::test]
async fn test_trades_successfull() {
    let bitso = Bitso::default()
        .build();
    let result = bitso.get_trades(Some("btc_mxn")).await;
    println!("{:?}", result);
}

/// Test unsuccessful request to get trades
#[tokio::test]
async fn test_trades_unsuccessfull() {
    let bitso = Bitso::default()
        .build();
    let result = bitso.get_trades(Some("CREATEERROR")).await;
    println!("{:?}", result);
}

#[test]
fn test_auth_headers() {
    let client_credentials = BitsoCredentials::default()
        .api_secret("secret")
        .api_key("key")
        .build();
    let bitso = Bitso::default()
        .client_credentials_manager(client_credentials)
        .build();
    let head = bitso.auth_headers(
        Method::GET,
        "path",
        None
    );
    println!("{}", head);
}
