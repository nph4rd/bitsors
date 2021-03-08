extern crate bitsors;

use bitsors::auth::BitsoCredentials;
use bitsors::client::{Bitso, OptionalOrderParams};

#[tokio::main]
async fn main() {
    // Set API_KEY and API_SECRET in .env file or
    // export API_KEY="your api_key"
    // export API_SECRET="your_api_secret"
    let client_credential = BitsoCredentials::default().build();
    // Or set API_KEY and API_SECRET explictly
    // let client_credential = BitsoCredentials::default()
    //     .api_key("this-is-my-client-id")
    //     .api_secret("this-is-my-client-secret")
    //     .build();
    let bitso = Bitso::default()
        .client_credentials_manager(client_credential)
        .build();
    let optional_order_params = OptionalOrderParams {
        major: Some("0.0001"),
        minor: None,
        price: None,
        stop: None,
        time_in_force: None,
        origin_id: None,
    };
    let result = bitso
        .place_order("btc_mxn", "sell", "market", Some(optional_order_params))
        .await;
    println!("{:?}", result);
}
