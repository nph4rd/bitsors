extern crate bitsors;
use std::collections::HashMap;

use bitsors::client::Bitso;

#[tokio::test]
async fn test_get() {
    let bitso = Bitso::default()
        .build();
    let _result = bitso.get("https://api.bitso.com/v3/available_books/", &mut HashMap::new()).await;
}
