use std::time::{SystemTime, UNIX_EPOCH};
use hex::encode;
use std::collections::HashMap;
use openssl::hash::MessageDigest;
use openssl::pkey::PKey;
use openssl::sign::Signer;
use reqwest::header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE};
use reqwest::Client;
use reqwest::Method;
use reqwest::StatusCode;
use serde::de::Deserialize;
use serde_json::Value;
use super::util::convert_map_to_string;
use super::auth::BitsoCredentials;
use std::borrow::Cow;
use std::fmt;
use super::model::public::*;
use super::model::private::*;

lazy_static! {
    /// HTTP Client
    pub static ref CLIENT: Client = Client::new();
}

const EMPTY_CREDENTIALS_MSG: &str = "You need to set your Bitso API \
                                     credentials. You can do this \
                                     by setting environment variables \
                                     in a `.env` file: \
                                     API_KEY=your api_key \
                                     API_SECRET=your_api_secret. \
                                     For more information visit: \
                                     `https://bitso.com/api_info#generating-api-keys`";


/// API Type
pub enum ApiType {
    Public,
    Private
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
            StatusCode::BAD_REQUEST => ApiError::RegularError{
                success: false,
                error: String::from("Bad request")
            },
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

    pub fn prefix(mut self, prefix: &str) -> Bitso {
        self.prefix = prefix.to_owned();
        self
    }

    pub fn client_credentials_manager(
        mut self,
        client_credential_manager: BitsoCredentials,
    ) -> Bitso {
        self.client_credentials_manager = Some(client_credential_manager);
        self
    }

    /// Build Bitso API object
    pub fn build(self) -> Bitso {
        self
    }

    pub fn auth_headers(
        &self,
        method: &Method,
        request_path: &str,
        payload: Option<&Value>,
    ) -> String {
        let payload_string: String;
        if method != Method::POST {
            payload_string = "".to_owned();
        } else {
            if let Some(json) = payload {
                payload_string = json.to_string();
            } else {
                panic!("POST method must have a payload.")
            }
        }

        let api_key = self
            .client_credentials_manager
            .as_ref()
            .unwrap()
            .get_key();
        let api_secret = self
            .client_credentials_manager
            .as_ref()
            .unwrap()
            .get_secret();

        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis()
            .to_string();
        let message = format!(
            "{}{}{}{}",
            nonce,
            method.as_str().to_owned(),
            request_path.to_owned(),
            payload_string
        );
        let key = PKey::hmac(
            api_secret.as_bytes()
        ).unwrap();
        let mut signer = Signer::new(
            MessageDigest::sha256(),
            &key
        ).unwrap();
        signer.update(message.as_bytes()).unwrap();
        let signature = encode(
            signer.sign_to_vec().unwrap()
        );
        format!(
            "Bitso {}:{}:{}",
            api_key,
            nonce,
            signature,
        )
    }

    async fn internal_call(
        &self,
        method: Method,
        url: &str,
        payload: Option<&Value>,
        api_type: ApiType,
    ) -> Result<String, failure::Error> {
        let mut url: Cow<str> = url.into();

        let mut headers = HeaderMap::new();
        if let ApiType::Private = api_type {
            headers.insert(
                AUTHORIZATION,
                self.auth_headers(
                    &method,
                    &url,
                    payload
                ).parse().unwrap()
            );
            headers.insert(
                CONTENT_TYPE,
                "application/json".parse().unwrap()
            );
        }

        if !url.starts_with("http") {
            url = [self.prefix.as_str(), &url]
                .concat()
                .into();
        }

        let response = {
            let mut builder = CLIENT.request(
                method,
                &url.into_owned()
            );
            if let ApiType::Private = api_type {
                builder = builder.headers(headers);
            }
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
        api_type: ApiType,
    ) -> Result<String, failure::Error> {
        if !params.is_empty() {
            let param: String = convert_map_to_string(params);
            let mut url_with_params = url.to_owned();
            url_with_params.push('?');
            url_with_params.push_str(&param);
            self.internal_call(
                Method::GET,
                &url_with_params,
                None,
                api_type
            ).await
        } else {
            self.internal_call(Method::GET, url, None, api_type).await
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
        let url = String::from("/v3/available_books/");
        let result = self.get(
            &url,
            &mut HashMap::new(),
            ApiType::Public
        ).await?;
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
        let url = String::from("/v3/ticker/");
        let result = self.get(
            &url,
            &mut params,
            ApiType::Public
        ).await?;
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
        let url = String::from("/v3/order_book/");
        let result = self.get(
            &url,
            &mut params,
            ApiType::Public
        ).await?;
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
        let url = String::from("/v3/trades/");
        let result = self.get(
            &url,
            &mut params,
            ApiType::Public
        ).await?;
        self.convert_result::<Trades>(&result)
    }


    ///
    /// Private API
    ///

    /// Make a get request to get account status
    /// https://bitso.com/api_info#account-status
    pub async fn get_account_status(
        &self,
    ) -> Result<AccountStatus, failure::Error> {
        let url = String::from("/v3/account_status/");
        let client_credentials = self.client_credentials_manager.as_ref();
        match client_credentials {
            Some(c) => {
                if c.get_key().is_empty() {
                    return Err(
                        failure::err_msg(EMPTY_CREDENTIALS_MSG)
                    )
                }
            },
            None => return Err(
                    failure::err_msg(EMPTY_CREDENTIALS_MSG)
                    ),
        }
        let result = self.get(
            &url,
            &mut HashMap::new(),
            ApiType::Private
        ).await?;
        self.convert_result::<AccountStatus>(&result)
    }

    /// Make a get request to get account balance
    /// https://bitso.com/api_info#account-balance
    pub async fn get_account_balance(
        &self,
    ) -> Result<AccountBalance, failure::Error> {
        let url = String::from("/v3/balance/");
        let client_credentials = self.client_credentials_manager.as_ref();
        match client_credentials {
            Some(c) => {
                if c.get_key().is_empty() {
                    return Err(
                        failure::err_msg(EMPTY_CREDENTIALS_MSG)
                    )
                }
            },
            None => return Err(
                    failure::err_msg(EMPTY_CREDENTIALS_MSG)
                    ),
        }
        let result = self.get(
            &url,
            &mut HashMap::new(),
            ApiType::Private
        ).await?;
        self.convert_result::<AccountBalance>(&result)
    }

    /// Make a get request to get fees
    /// https://bitso.com/api_info#fees
    pub async fn get_fees(
        &self,
    ) -> Result<Fees, failure::Error> {
        let url = String::from("/v3/fees/");
        let client_credentials = self.client_credentials_manager.as_ref();
        match client_credentials {
            Some(c) => {
                if c.get_key().is_empty() {
                    return Err(
                        failure::err_msg(EMPTY_CREDENTIALS_MSG)
                    )
                }
            },
            None => return Err(
                    failure::err_msg(EMPTY_CREDENTIALS_MSG)
                    ),
        }
        let result = self.get(
            &url,
            &mut HashMap::new(),
            ApiType::Private
        ).await?;
        self.convert_result::<Fees>(&result)
    }

    /// Make a get request to get fees
    /// https://bitso.com/api_info#ledger
    pub async fn get_ledger(
        &self,
    ) -> Result<Ledger, failure::Error> {
        let url = String::from("/v3/ledger/");
        let client_credentials = self.client_credentials_manager.as_ref();
        match client_credentials {
            Some(c) => {
                if c.get_key().is_empty() {
                    return Err(
                        failure::err_msg(EMPTY_CREDENTIALS_MSG)
                    )
                }
            },
            None => return Err(
                    failure::err_msg(EMPTY_CREDENTIALS_MSG)
                    ),
        }
        let result = self.get(
            &url,
            &mut HashMap::new(),
            ApiType::Private
        ).await?;
        self.convert_result::<Ledger>(&result)
    }

    /// Make a get request to get fees
    /// https://bitso.com/api_info#withdrawals
    pub async fn get_withdrawals(
        &self,
    ) -> Result<Withdrawals, failure::Error> {
        let url = String::from("/v3/withdrawals/");
        let client_credentials = self.client_credentials_manager.as_ref();
        match client_credentials {
            Some(c) => {
                if c.get_key().is_empty() {
                    return Err(
                        failure::err_msg(EMPTY_CREDENTIALS_MSG)
                    )
                }
            },
            None => return Err(
                    failure::err_msg(EMPTY_CREDENTIALS_MSG)
                    ),
        }
        let result = self.get(
            &url,
            &mut HashMap::new(),
            ApiType::Private
        ).await?;
        self.convert_result::<Withdrawals>(&result)
    }

    /// Make a get request to get fees
    /// https://bitso.com/api_info#fundings
    pub async fn get_fundings(
        &self,
    ) -> Result<Fundings, failure::Error> {
        let url = String::from("/v3/fundings/");
        let client_credentials = self.client_credentials_manager.as_ref();
        match client_credentials {
            Some(c) => {
                if c.get_key().is_empty() {
                    return Err(
                        failure::err_msg(EMPTY_CREDENTIALS_MSG)
                    )
                }
            },
            None => return Err(
                    failure::err_msg(EMPTY_CREDENTIALS_MSG)
                    ),
        }
        let result = self.get(
            &url,
            &mut HashMap::new(),
            ApiType::Private
        ).await?;
        self.convert_result::<Fundings>(&result)
    }

    /// Make a get request to get fees
    /// https://bitso.com/api_info#user-trades
    pub async fn get_user_trades(
        &self,
    ) -> Result<UserTrades, failure::Error> {
        let url = String::from("/v3/user_trades/");
        let client_credentials = self.client_credentials_manager.as_ref();
        match client_credentials {
            Some(c) => {
                if c.get_key().is_empty() {
                    return Err(
                        failure::err_msg(EMPTY_CREDENTIALS_MSG)
                    )
                }
            },
            None => return Err(
                    failure::err_msg(EMPTY_CREDENTIALS_MSG)
                    ),
        }
        let result = self.get(
            &url,
            &mut HashMap::new(),
            ApiType::Private
        ).await?;
        self.convert_result::<UserTrades>(&result)
    }

    /// Make a get request to get fees
    /// https://bitso.com/api_info#order-trades
    pub async fn get_order_trades(
        &self,
        oid: &str,
    ) -> Result<OrderTrades, failure::Error> {
        let url = format!("/v3/order_trades/{}/", oid.to_owned());
        let client_credentials = self.client_credentials_manager.as_ref();
        match client_credentials {
            Some(c) => {
                if c.get_key().is_empty() {
                    return Err(
                        failure::err_msg(EMPTY_CREDENTIALS_MSG)
                    )
                }
            },
            None => return Err(
                    failure::err_msg(EMPTY_CREDENTIALS_MSG)
                    ),
        }
        let result = self.get(
            &url,
            &mut HashMap::new(),
            ApiType::Private
        ).await?;
        self.convert_result::<OrderTrades>(&result)
    }

    /// Make a get request to get fees
    /// https://bitso.com/api_info#open-orders
    pub async fn get_open_orders(
        &self,
        book: Option<&str>,
    ) -> Result<OpenOrders, failure::Error> {
        let mut url = String::from("/v3/open_orders");
        if let Some(_book) = book {
            url = format!(
                "{}?book={}",
                url,
                _book.to_owned()
            );
        };
        let client_credentials = self.client_credentials_manager.as_ref();
        match client_credentials {
            Some(c) => {
                if c.get_key().is_empty() {
                    return Err(
                        failure::err_msg(EMPTY_CREDENTIALS_MSG)
                    )
                }
            },
            None => return Err(
                    failure::err_msg(EMPTY_CREDENTIALS_MSG)
                    ),
        }
        let result = self.get(
            &url,
            &mut HashMap::new(),
            ApiType::Private
        ).await?;
        self.convert_result::<OpenOrders>(&result)
    }
}

