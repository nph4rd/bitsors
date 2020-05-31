extern crate bitsors;

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
