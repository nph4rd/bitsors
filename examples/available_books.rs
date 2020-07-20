extern crate bitsors;

use bitsors::client::Bitso;

#[tokio::main]
async fn main() {
    let bitso = Bitso::default()
        .build();
    let result = bitso.get_available_books().await;
    println!("{:?}", result);
}
