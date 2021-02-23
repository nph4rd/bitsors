extern crate bitsors;

use bitsors::client::{Bitso, OptionalParams};

#[tokio::main]
async fn main() {
    let bitso = Bitso::default().build();
    let optional_params = OptionalParams {
        marker: None,
        sort: None,
        limit: None,
    };
    let result = bitso.get_trades("btc_mxn", optional_params).await;
    println!("{:?}", result);
}
