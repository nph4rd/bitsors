use std::collections::HashMap;
use reqwest::Client;
use reqwest::Method;
use reqwest::StatusCode;
use serde::de::Deserialize;
use serde_json::Value;
use super::util::convert_map_to_string;
use super::auth::BitsoCredentials;
use std::borrow::Cow;
use std::fmt;
use super::model::public::{AvailableBooks, Ticker, OrderBook, Trades};

lazy_static! {
    /// HTTP Client
    pub static ref CLIENT: Client = Client::new();
}

/// API Errors
#[derive(Debug, Deserialize)]
pub enum ApiError {
    #[serde(alias = "error")]
    RegularError {
        success: bool,
        error: String,
    },
    Other(u16),
}

impl failure::Fail for ApiError {}
impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ApiError::RegularError { success, error } => {
                write!(
                    f,
                    "Bitso API error code {}: {}",
                    success,
                    error
                )
            },
            ApiError::Other(s) => {
                write!(
                    f,
                    "Bitso API reported error code {}",
                    s
                )
            },
        }
    }
}
impl ApiError {
    async fn from_response(
        response: reqwest::Response
    ) -> Self {
        match response.status() {
            StatusCode::BAD_REQUEST => ApiError::RegularError{success: false, error: String::from("Bad request")},
            status => ApiError::Other(status.as_u16()),
        }
    }
}

/// Bitso API object
pub struct Bitso {
    pub prefix: String,
    pub client_credentials_manager: Option<BitsoCredentials>,
}

impl Bitso {

    /// Bitso instance
    pub fn default() -> Bitso {
        Bitso {
            prefix: "https://api.bitso.com".to_owned(),
            client_credentials_manager: None,
        }
    }

    /// Build Bitso API object
    pub fn build(self) -> Bitso {
        self
    }

    async fn internal_call(
        &self,
        method: Method,
        url: &str,
        payload: Option<&Value>,
    ) -> Result<String, failure::Error> {
        let mut url: Cow<str> = url.into();
        if !url.starts_with("http") {
            url = ["https://api.bitso.com/v3/", &url]
                .concat()
                .into();
        }

        let response = {
            let builder = CLIENT.request(
                method,
                &url.into_owned()
            );
            let builder = if let Some(json) = payload {
                builder.json(json)
            } else {
                builder
            };
            builder.send().await?
        };

        if response.status().is_success() {
            match response.text().await {
                Ok(text) => Ok(text),
                Err(e) => Err(failure::err_msg(format!(
                    "Error getting text out of response {}",
                    e
                ))),
            }
        } else {
            Err(failure::Error::from(
                    ApiError::from_response(response).await,
            ))
        }
    }

    /// Function to make get requests
    async fn get(
        &self,
        url:&str,
        params: &mut HashMap<String, String>,
    ) -> Result<String, failure::Error> {
        if !params.is_empty() {
            let param: String = convert_map_to_string(params);
            let mut url_with_params = url.to_owned();
            url_with_params.push('?');
            url_with_params.push_str(&param);
            self.internal_call(
                Method::GET,
                &url_with_params,
                None
            ).await
        } else {
            self.internal_call(Method::GET, url, None).await
        }
    }

    /// Function to convert result to models
    pub fn convert_result<'a, T: Deserialize<'a>>(
        &self,
        input: &'a str,
    ) -> Result<T, failure::Error> {
        let result = serde_json::from_str::<T>(input).map_err(
            |e| {
                format_err!(
                    "Convert result failed, reason: {:?}; content: [{:?}]",
                    e,
                    input
                )
            }
        )?;
        Ok(result)
    }

    /// Make a get request to pull available books
    /// https://bitso.com/api_info/#available-books
    pub async fn get_available_books(
        &self
    ) -> Result<AvailableBooks, failure::Error> {
        let url = String::from("available_books/");
        let result = self.get(&url, &mut HashMap::new()).await?;
        self.convert_result::<AvailableBooks>(&result)
    }

    /// Make a get request to pull ticker
    /// https://bitso.com/api_info/#ticker
    pub async fn get_ticker(
        &self,
        book: Option<&str>,
    ) -> Result<Ticker, failure::Error> {
        let mut params = HashMap::new();
        if let Some(_book) = book {
            params.insert("book".to_owned(), _book.to_string());
        }
        let url = String::from("ticker/");
        let result = self.get(&url, &mut params).await?;
        self.convert_result::<Ticker>(&result)
    }

    /// Make a get request to pull a specific order book
    /// https://bitso.com/api_info/#order-book
    pub async fn get_order_book(
        &self,
        book: Option<&str>,
    ) -> Result<OrderBook, failure::Error> {
        let mut params = HashMap::new();
        if let Some(_book) = book {
            params.insert("book".to_owned(), _book.to_string());
        }
        let url = String::from("order_book/");
        let result = self.get(&url, &mut params).await?;
        self.convert_result::<OrderBook>(&result)
    }

    /// Make a get request to pull a specific trade
    /// https://bitso.com/api_info/#trades
    pub async fn get_trades(
        &self,
        book: Option<&str>,
    ) -> Result<Trades, failure::Error> {
        let mut params = HashMap::new();
        if let Some(_book) = book {
            params.insert("book".to_owned(), _book.to_string());
        }
        let url = String::from("trades/");
        let result = self.get(&url, &mut params).await?;
        self.convert_result::<Trades>(&result)
    }
}

