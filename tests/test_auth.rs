extern crate bitsors;

use bitsors::auth::BitsoCredentials;

/// Test credential set-up
#[test]
fn test_credentials_setup() {
    let auth = BitsoCredentials::default();
    println!("{:?}", auth);
}
