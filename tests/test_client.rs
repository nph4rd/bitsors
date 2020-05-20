extern crate bitsors;

use bitsors::client::Bitso;

#[tokio::test]
async fn test_available_books() {
    let bitso = Bitso::default()
        .build();
    let result = bitso.get_available_books().await;
    println!("{:?}", result);
}

#[tokio::test]
async fn test_ticker() {
    let bitso = Bitso::default()
        .build();
    let result = bitso.get_ticker().await;
    println!("{:?}", result);
}

#[tokio::test]
async fn test_order_book() {
    let bitso = Bitso::default()
        .build();
    let result = bitso.get_order_book(Some("btc_mxn")).await;
    println!("{:?}", result);
}

#[tokio::test]
async fn test_trades() {
    let bitso = Bitso::default()
        .build();
    let result = bitso.get_trades(Some("btc_mxn")).await;
    println!("{:?}", result);
}

