extern crate bitsors;

use bitsors::client::Bitso;

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
    assert!(result.is_err());
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
    assert!(result.is_err());
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
    assert!(result.is_err());
    println!("{:?}", result);
}

