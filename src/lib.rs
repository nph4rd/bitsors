//! ## Description
//! `bitsors` is a wrapper for the [Bitso API](https://bitso.com/api_info). So far
//! it includes most of the functionality of the [public](https://bitso.com/api_info#public-rest-api)
//! and [private](https://bitso.com/api_info#private-rest-api) API endpoints.
//!
//! __Disclaimer__: Some of the functionality for the private API is not covered in
//! this wrapper.
//!
//! ## Getting started
//! ### Authorization
//! No authorization is needed for the public API. However, the private API
//! needs authorization as indicated in the [official API documentation](https://bitso.com/api_info#generating-api-keys).
//! The wrapper takes care of the authentication method. To be able to use
//! the private API functionality, therefore, you only need to set your
//! API key and secret variables in a `.env` file, export them directly
//! to your environment variables or supply them in-line through the
//! `BitsoCredentials`' `api_key` and `api_secret` methods.
//!
//! ## Examples
//! For more detailed information on how to use this wrapper, you could
//! see the [examples](https://github.com/arturomf94/bitsors/tree/master/examples), which hold detailed use-cases.

extern crate openssl;
extern crate hex;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;
extern crate dotenv;
#[macro_use]
extern crate failure;
pub mod client;
pub mod util;
pub mod model;
pub mod auth;
