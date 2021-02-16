extern crate bitsors;

use bitsors::client::Bitso;

#[tokio::main]
async fn main() {
    let bitso = Bitso::default().build();
    let result = bitso.get_order_book("btc_mxn", false).await;
    println!("{:?}", result);
}
