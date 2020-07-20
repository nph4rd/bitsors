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
use super::model::JSONResponse;

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


/// API Type that indicates whether a method
/// corresponds to the public or private API.
pub enum ApiType {
    Public,
    Private
}

/// API Errors associated to the Bitso API object
#[derive(Debug, Deserialize)]
pub enum ApiError {
    /// A regular error is derived from
    /// Bitso's API responses. For more information
    /// see: `https://bitso.com/api_info#error-codes`
    #[serde(alias = "error")]
    RegularError {
        success: bool,
        code: String,
        message: String,
    },
    Other(u16),
}

impl failure::Fail for ApiError {}
impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ApiError::RegularError {
                success: _,
                code,
                message
            } => {
                write!(
                    f,
                    "Bitso API error code {}: {}",
                    code,
                    message
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

/// A regular error from Bitso's API.
/// See: `https://bitso.com/api_info#error-codes`
#[derive(Debug, Deserialize)]
pub struct RegularError {
    pub success: bool,
    pub error: ErrorDetails,
}

/// See: `https://bitso.com/api_info#error-codes`
#[derive(Debug, Deserialize)]
pub struct ErrorDetails {
    pub code: String,
    pub message: String,
}

impl ApiError {
    async fn from_response(
        response: reqwest::Response
    ) -> Self {
        match response.status() {
            StatusCode::BAD_REQUEST => {
                let error = response
                    .json::<RegularError>()
                    .await
                    .unwrap();
                ApiError::RegularError{
                    success: error.success,
                    code: error.error.code,
                    message: error.error.message,
                }
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

    /// Set prefix
    pub fn prefix(mut self, prefix: &str) -> Bitso {
        self.prefix = prefix.to_owned();
        self
    }

    /// Set client credentials
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

    /// Construct authorization headers.
    /// See: `https://bitso.com/api_info#creating-and-signing-requests`
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
            if let Some(json) = payload {
                builder = builder.json(json);
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

    /// Makes get requests
    async fn get(
        &self,
        url: &str,
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

    /// Makes post requests
    async fn post(
        &self,
        url: &str,
        payload: &Value,
        api_type: ApiType,
    ) -> Result<String, failure::Error> {
        self.internal_call(
            Method::POST,
            url,
            Some(payload),
            api_type
        ).await
    }

    /// Makes delete requests
    async fn delete(
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
                Method::DELETE,
                &url_with_params,
                None,
                api_type
            ).await
        } else {
            self.internal_call(Method::DELETE, url, None, api_type).await
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

    /// Make a request to get available books
    /// https://bitso.com/api_info/#available-books
    pub async fn get_available_books(
        &self
    ) -> Result<JSONResponse<Vec<AvailableBook>>, failure::Error> {
        let url = String::from("/v3/available_books/");
        let result = self.get(
            &url,
            &mut HashMap::new(),
            ApiType::Public
        ).await?;
        self.convert_result::<JSONResponse<Vec<AvailableBook>>>(&result)
    }

    /// Make a request to get ticker
    /// https://bitso.com/api_info/#ticker
    pub async fn get_ticker(
        &self,
        book: &str,
    ) -> Result<JSONResponse<BookTicker>, failure::Error> {
        let mut params = HashMap::new();
        params.insert("book".to_owned(), book.to_string());
        let url = String::from("/v3/ticker/");
        let result = self.get(
            &url,
            &mut params,
            ApiType::Public
        ).await?;
        self.convert_result::<JSONResponse<BookTicker>>(&result)
    }

    /// Make a request to get a specific order book
    /// https://bitso.com/api_info/#order-book
    pub async fn get_order_book(
        &self,
        book: &str,
    ) -> Result<JSONResponse<OrderBookPayload>, failure::Error> {
        let mut params = HashMap::new();
        params.insert("book".to_owned(), book.to_string());
        let url = String::from("/v3/order_book/");
        let result = self.get(
            &url,
            &mut params,
            ApiType::Public
        ).await?;
        self.convert_result::<JSONResponse<OrderBookPayload>>(&result)
    }

    /// Make a request to get a specific trade
    /// https://bitso.com/api_info/#trades
    pub async fn get_trades(
        &self,
        book: &str,
    ) -> Result<JSONResponse<Vec<Trade>>, failure::Error> {
        let mut params = HashMap::new();
        params.insert("book".to_owned(), book.to_string());
        let url = String::from("/v3/trades/");
        let result = self.get(
            &url,
            &mut params,
            ApiType::Public
        ).await?;
        self.convert_result::<JSONResponse<Vec<Trade>>>(&result)
    }

    /// Make a request to get account status
    /// https://bitso.com/api_info#account-status
    pub async fn get_account_status(
        &self,
    ) -> Result<JSONResponse<AccountStatusPayload>, failure::Error> {
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
        self.convert_result::<JSONResponse<AccountStatusPayload>>(&result)
    }

    /// Make a request to get account balance
    /// https://bitso.com/api_info#account-balance
    pub async fn get_account_balance(
        &self,
    ) -> Result<JSONResponse<Balances>, failure::Error> {
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
        self.convert_result::<JSONResponse<Balances>>(&result)
    }

    /// Make a request to get fees
    /// https://bitso.com/api_info#fees
    pub async fn get_fees(
        &self,
    ) -> Result<JSONResponse<FeesPayload>, failure::Error> {
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
        self.convert_result::<JSONResponse<FeesPayload>>(&result)
    }

    /// Make a request to get ledger
    /// https://bitso.com/api_info#ledger
    pub async fn get_ledger(
        &self,
    ) -> Result<JSONResponse<Vec<LedgerInstance>>, failure::Error> {
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
        self.convert_result::<JSONResponse<Vec<LedgerInstance>>>(&result)
    }

    /// Make a request to get withdrawals
    /// https://bitso.com/api_info#withdrawals
    pub async fn get_withdrawals(
        &self,
    ) -> Result<JSONResponse<Vec<WithdrawalsPayload>>, failure::Error> {
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
        self.convert_result::<JSONResponse<Vec<WithdrawalsPayload>>>(&result)
    }

    /// Make a request to get fundings
    /// https://bitso.com/api_info#fundings
    pub async fn get_fundings(
        &self,
    ) -> Result<JSONResponse<Vec<FundingsPayload>>, failure::Error> {
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
        self.convert_result::<JSONResponse<Vec<FundingsPayload>>>(&result)
    }

    /// Make a request to get user trades
    /// https://bitso.com/api_info#user-trades
    pub async fn get_user_trades(
        &self,
    ) -> Result<JSONResponse<Vec<UserTradesPayload>>, failure::Error> {
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
        self.convert_result::<JSONResponse<Vec<UserTradesPayload>>>(&result)
    }

    /// Make a request to get order trades
    /// https://bitso.com/api_info#order-trades
    pub async fn get_order_trades(
        &self,
        oid: &str,
    ) -> Result<JSONResponse<Vec<OrderTradesPayload>>, failure::Error> {
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
        self.convert_result::<JSONResponse<Vec<OrderTradesPayload>>>(&result)
    }

    /// Make a request to get open orders
    /// https://bitso.com/api_info#open-orders
    pub async fn get_open_orders(
        &self,
        book: &str,
    ) -> Result<JSONResponse<Vec<OpenOrdersPayload>>, failure::Error> {
        let url = format!(
            "/v3/open_orders?book={}",
            book.to_owned()
        );
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
        self.convert_result::<JSONResponse<Vec<OpenOrdersPayload>>>(&result)
    }

    /// Make a request to get lookup orders
    /// https://bitso.com/api_info#lookup-orders
    pub async fn get_lookup_orders(
        &self,
        oid: &str,
    ) -> Result<JSONResponse<Vec<LookupOrdersPayload>>, failure::Error> {
        let url = format!("/v3/orders/{}/", oid.to_owned());
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
        self.convert_result::<JSONResponse<Vec<LookupOrdersPayload>>>(&result)
    }

    /// Make a request to cancel order
    /// https://bitso.com/api_info#cancel-order
    pub async fn cancel_order(
        &self,
        oid: &str,
    ) -> Result<JSONResponse<Vec<String>>, failure::Error> {
        let url = format!("/v3/orders/{}/", oid.to_owned());
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
        let result = self.delete(
            &url,
            &mut HashMap::new(),
            ApiType::Private
        ).await?;
        self.convert_result::<JSONResponse<Vec<String>>>(&result)
    }

    /// Make a post request to place an order
    /// https://bitso.com/api_info#place-an-order
    pub async fn place_order(
        &self,
        book: &str,
        side: &str,
        r#type: &str,
        major: Option<&str>,
    ) -> Result<JSONResponse<PlaceOrderPayload>, failure::Error> {
        let url = String::from("/v3/orders/");
        let params = json!({
            "book": book,
            "side": side,
            "type": r#type,
            "major": major
        });
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
        let result = self.post(
            &url,
            &params,
            ApiType::Private
        ).await?;
        self.convert_result::<JSONResponse<PlaceOrderPayload>>(&result)
    }

    /// Make a request to get lookup orders
    /// https://bitso.com/api_info#lookup-orders
    pub async fn get_funding_destination(
        &self,
        fund_currency: &str,
    ) -> Result<JSONResponse<FundingDestination>, failure::Error> {
        let url = String::from("/v3/funding_destination/");
        let mut params = HashMap::new();
        params.insert("fund_currency".to_owned(), fund_currency.to_string());
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
            &mut params,
            ApiType::Private
        ).await?;
        self.convert_result::<JSONResponse<FundingDestination>>(&result)
    }

    /// Make a request to place an crypto withdrawal
    /// https://bitso.com/api_info#crypto-withdrawals
    pub async fn crypto_withdrawal(
        &self,
        currency: &str,
        amount: &str,
        address: &str,
        max_fee: Option<&str>,
        destination_tag: Option<&str>,
    ) -> Result<JSONResponse<Withdrawal<CryptoWithdrawal>>, failure::Error> {
        let url = String::from("/v3/crypto_withdrawal/");
        let params = json!({
            "currency": currency,
            "amount": amount,
            "address": address,
            "max_fee": max_fee,
            "destination_tag": destination_tag,
        });
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
        let result = self.post(
            &url,
            &params,
            ApiType::Private
        ).await?;
        self.convert_result::<JSONResponse<Withdrawal<CryptoWithdrawal>>>(&result)
    }

    /// Make a request to place a speri withdrawal
    /// https://bitso.com/api_info#spei-withdrawal
    pub async fn spei_withdrawal(
        &self,
        amount: &str,
        recipient_given_names: &str,
        recipient_family_names: &str,
        clabe: &str,
        notes_ref: Option<&str>,
        numeric_ref: Option<&str>
    ) -> Result<JSONResponse<Withdrawal<SPEIWithdrawal>>, failure::Error> {
        let url = String::from("/v3/spei_withdrawal/");
        let params = json!({
            "amount": amount,
            "recipient_given_names": recipient_given_names,
            "recipient_family_names": recipient_family_names,
            "clabe": clabe,
            "notes_ref": notes_ref,
            "numeric_ref": numeric_ref
        });
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
        let result = self.post(
            &url,
            &params,
            ApiType::Private
        ).await?;
        self.convert_result::<JSONResponse<Withdrawal<SPEIWithdrawal>>>(&result)
    }

    /// Make a request to get bank codes
    /// https://bitso.com/api_info#bank-codes
    pub async fn get_bank_codes(
        &self,
    ) -> Result<JSONResponse<Vec<BankCode>>, failure::Error> {
        let url = String::from("/v3/mx_bank_codes/");
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
        self.convert_result::<JSONResponse<Vec<BankCode>>>(&result)
    }

    /// Make a post request to make a debit-card ithdrawal
    /// https://bitso.com/api_info#debit-card-withdrawal
    pub async fn debit_card_withdrawal(
        &self,
        amount: &str,
        recipient_given_names: &str,
        recipient_family_names: &str,
        card_number: &str,
        bank_code: &str,
    ) -> Result<JSONResponse<Withdrawal<DebitWithdrawal>>, failure::Error> {
        let url = String::from("/v3/debit_card_withdrawal/");
        let params = json!({
            "amount": amount,
            "recipient_given_names": recipient_given_names,
            "recipient_family_names": recipient_family_names,
            "card_number": card_number,
            "bank_code": bank_code
        });
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
        let result = self.post(
            &url,
            &params,
            ApiType::Private
        ).await?;
        self.convert_result::<JSONResponse<Withdrawal<DebitWithdrawal>>>(&result)
    }

    /// Make a post request to make a phone-number withdrawal
    /// https://bitso.com/api_info#phone-number-withdrawal
    pub async fn phone_number_withdrawal(
        &self,
        amount: &str,
        recipient_given_names: &str,
        recipient_family_names: &str,
        phone_number: &str,
        bank_code: &str,
    ) -> Result<JSONResponse<Withdrawal<PhoneWithdrawal>>, failure::Error> {
        let url = String::from("/v3/phone_withdrawal/");
        let params = json!({
            "amount": amount,
            "recipient_given_names": recipient_given_names,
            "recipient_family_names": recipient_family_names,
            "phone_number": phone_number,
            "bank_code": bank_code
        });
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
        let result = self.post(
            &url,
            &params,
            ApiType::Private
        ).await?;
        self.convert_result::<JSONResponse<Withdrawal<PhoneWithdrawal>>>(&result)
    }
}

