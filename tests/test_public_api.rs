extern crate bitsors;
extern crate mockito;

use bitsors::client::{Bitso, OptionalParams};
use mockito::{mock, Matcher};

/// Test unsuccesful request and error parsing
#[tokio::test]
async fn test_error_parsing() {
    let _mock = mock("GET", "/v3/ticker/")
        .match_query(Matcher::UrlEncoded("book".into(), "FAKEORDERBOOK".into()))
        .with_status(400)
        .with_body(
            r#"{
            "success": false,
            "error": {
                "code": "0301",
                "message": "Unknown OrderBook FAKEORDERBOOK"
            }
        }"#,
        )
        .create();
    let bitso = Bitso::default()
        .prefix(mockito::server_url().as_str())
        .build();
    let result = bitso.get_ticker("FAKEORDERBOOK").await;
    assert!(result.is_err());
    println!("{:?}", result);
    assert_eq!(
        result.unwrap_err().to_string(),
        "Bitso API error code 0301: Unknown OrderBook FAKEORDERBOOK"
    );
}

/// Test successful request to get available books
#[tokio::test]
async fn test_available_books() {
    let _mock = mock("GET", "/v3/available_books/")
        .with_status(200)
        .with_body(
            r#"{
            "success": true,
            "payload": [{
                "book": "btc_mxn",
                "minimum_amount": ".003",
                "maximum_amount": "1000.00",
                "minimum_price": "100.00",
                "maximum_price": "1000000.00",
                "minimum_value": "25.00",
                "maximum_value": "1000000.00"
            }, {
                "book": "eth_mxn",
                "minimum_amount": ".003",
                "maximum_amount": "1000.00",
                "minimum_price": "100.0",
                "maximum_price": "1000000.0",
                "minimum_value": "25.0",
                "maximum_value": "1000000.0"
            }]
        }"#,
        )
        .create();
    let bitso = Bitso::default()
        .prefix(mockito::server_url().as_str())
        .build();
    let result = bitso.get_available_books().await;
    assert!(result.is_ok());
    println!("{:?}", result);
}

/// Test successful request to get ticker
#[tokio::test]
async fn test_ticker() {
    let _mock = mock("GET", "/v3/ticker/")
        .match_query(Matcher::UrlEncoded("book".into(), "btc_mxn".into()))
        .with_status(200)
        .with_body(
            r#"{
            "success": true,
            "payload": {
                "book": "btc_mxn",
                "volume": "22.31349615",
                "high": "5750.00",
                "last": "5633.98",
                "low": "5450.00",
                "vwap": "5393.45",
                "ask": "5632.24",
                "bid": "5520.01",
                "created_at": "2016-04-08T17:52:31.000+00:00"
            }
        }"#,
        )
        .create();
    let bitso = Bitso::default()
        .prefix(mockito::server_url().as_str())
        .build();
    let result = bitso.get_ticker("btc_mxn").await;
    assert!(result.is_ok());
    println!("{:?}", result);
}

/// Test successful request to get order book
#[tokio::test]
async fn test_order_book() {
    let _mock = mock("GET", "/v3/order_book/")
        .match_query(Matcher::AllOf(vec![
            Matcher::UrlEncoded("book".into(), "btc_mxn".into()),
            Matcher::UrlEncoded("aggregate".into(), "false".into()),
        ]))
        .with_status(200)
        .with_body(
            r#"{
            "success": true,
            "payload": {
                "asks": [{
                    "book": "btc_mxn",
                    "price": "5632.24",
                    "amount": "1.34491802"
                },{
                    "book": "btc_mxn",
                    "price": "5633.44",
                    "amount": "0.4259"
                },{
                    "book": "btc_mxn",
                    "price": "5642.14",
                    "amount": "1.21642"
                }],
                "bids": [{
                    "book": "btc_mxn",
                    "price": "6123.55",
                    "amount": "1.12560000"
                },{
                    "book": "btc_mxn",
                    "price": "6121.55",
                    "amount": "2.23976"
                }],
                "updated_at": "2016-04-08T17:52:31.000+00:00",
                "sequence": "27214"
            }
        }"#,
        )
        .create();
    let bitso = Bitso::default()
        .prefix(mockito::server_url().as_str())
        .build();
    let result = bitso.get_order_book("btc_mxn", false).await;
    assert!(result.is_ok());
    println!("{:?}", result);
}

/// Test successful request to get order book, with aggregate data
#[tokio::test]
async fn test_order_book_with_optional_params() {
    let _mock = mock("GET", "/v3/order_book/")
        .match_query(Matcher::AllOf(vec![
            Matcher::UrlEncoded("book".into(), "btc_mxn".into()),
            Matcher::UrlEncoded("aggregate".into(), "true".into()),
        ]))
        .with_status(200)
        .with_body(
            r#"{
            "success": true,
            "payload": {
                "asks": [{
                    "book": "btc_mxn",
                    "price": "5632.24",
                    "amount": "1.34491802"
                },{
                    "book": "btc_mxn",
                    "price": "5633.44",
                    "amount": "0.4259"
                },{
                    "book": "btc_mxn",
                    "price": "5642.14",
                    "amount": "1.21642"
                }],
                "bids": [{
                    "book": "btc_mxn",
                    "price": "6123.55",
                    "amount": "1.12560000"
                },{
                    "book": "btc_mxn",
                    "price": "6121.55",
                    "amount": "2.23976"
                }],
                "updated_at": "2016-04-08T17:52:31.000+00:00",
                "sequence": "27214"
            }
        }"#,
        )
        .create();
    let bitso = Bitso::default()
        .prefix(mockito::server_url().as_str())
        .build();
    let result = bitso.get_order_book("btc_mxn", true).await;
    assert!(result.is_ok());
    println!("{:?}", result);
}

/// Test successful request to get trades
#[tokio::test]
async fn test_trades() {
    let _mock = mock("GET", "/v3/trades/")
        .match_query(Matcher::UrlEncoded("book".into(), "btc_mxn".into()))
        .with_status(200)
        .with_body(
            r#"{
            "success": true,
            "payload": [{
                "book": "btc_mxn",
                "created_at": "2016-04-08T17:52:31.000+00:00",
                "amount": "0.02000000",
                "maker_side": "buy",
                "price": "5545.01",
                "tid": 55845
            }, {
                "book": "btc_mxn",
                "created_at": "2016-04-08T17:52:31.000+00:00",
                "amount": "0.33723939",
                "maker_side": "sell",
                "price": "5633.98",
                "tid": 55844
            }]
        }"#,
        )
        .create();
    let bitso = Bitso::default()
        .prefix(mockito::server_url().as_str())
        .build();
    let result = bitso.get_trades("btc_mxn", None).await;
    assert!(result.is_ok());
    println!("{:?}", result);
}

/// Test successful request to get trades with optional parameters
#[tokio::test]
async fn test_trades_with_optional_params() {
    let _mock = mock("GET", "/v3/trades/")
        .match_query(Matcher::AllOf(vec![
            Matcher::UrlEncoded("book".into(), "btc_mxn".into()),
            Matcher::UrlEncoded("marker".into(), "55844".into()),
            Matcher::UrlEncoded("sort".into(), "asc".into()),
            Matcher::UrlEncoded("limit".into(), "1".into()),
        ]))
        .match_query(Matcher::UrlEncoded("book".into(), "btc_mxn".into()))
        .with_status(200)
        .with_body(
            r#"{
            "success": true,
            "payload": [{
                "book": "btc_mxn",
                "created_at": "2016-04-08T17:52:31.000+00:00",
                "amount": "0.02000000",
                "maker_side": "buy",
                "price": "5545.01",
                "tid": 55845
            }]
        }"#,
        )
        .create();
    let bitso = Bitso::default()
        .prefix(mockito::server_url().as_str())
        .build();
    let optional_params = OptionalParams {
        marker: Some(&55844),
        sort: Some("asc"),
        limit: Some(&1),
    };
    let result = bitso.get_trades("btc_mxn", Some(optional_params)).await;
    assert!(result.is_ok());
    println!("{:?}", result);
}

/// Test proper handling of errors external to the library
#[tokio::test]
async fn extern_errors() {
    let bitso = Bitso::default().prefix("bad prefix").build();
    let result = bitso.get_ticker("eth_mxn").await;
    println!("{:?}", result);
    assert_eq!(
        result.unwrap_err().to_string(),
        "builder error: relative URL without a base"
    );
}
