use std::collections::HashMap;
use reqwest::Client;
use reqwest::Method;
use super::util::convert_map_to_string;

lazy_static! {
    /// HTTP Client
    pub static ref CLIENT: Client = Client::new();
}

/// Bitso API object
pub struct Bitso {  }

impl Bitso {

    /// Bitso instance
    pub fn default() -> Bitso {
        Bitso {  }
    }

    /// Build Bitso API object
    pub fn build(self) -> Bitso {
        self
    }

    /// Function to make get requests
    pub async fn get(
        &self,
        url: &str,
        params: &mut HashMap<String, String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if !params.is_empty() {
            let param: String = convert_map_to_string(params);
            let mut url_with_params = url.to_owned();
            url_with_params.push('?');
            url_with_params.push_str(&param);
            let response = {
                let builder = CLIENT.request(Method::GET, &url_with_params);
                builder.send().await?
            };
            if response.status().is_success() {
                let body = response.text().await?;
                println!("{:?}", body);
            }
            Ok(())
        } else {
            let response = {
                let builder = CLIENT.request(Method::GET, url);
                builder.send().await?
            };
            if response.status().is_success() {
                let body = response.text().await?;
                println!("{:?}", body);
            }
            Ok(())
        }
    }

    /// Make a get request to pull available books
    /// https://bitso.com/api_info/#available-books
    pub async fn get_available_books(
        &self
    ) -> Result<(), Box<dyn std::error::Error>> {
        let url = String::from("https://api.bitso.com/v3/available_books/");
        self.get(&url, &mut HashMap::new()).await
    }

    /// Make a get request to pull ticker
    /// https://bitso.com/api_info/#ticker
    pub async fn get_ticker(
        &self
    ) -> Result<(), Box<dyn std::error::Error>> {
        let url = String::from("https://api.bitso.com/v3/ticker/");
        self.get(&url, &mut HashMap::new()).await
    }

    /// Make a get request to pull a specific order book
    /// https://bitso.com/api_info/#order-book
    pub async fn get_order_book(
        &self,
        order_book: Option<&str>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut params = HashMap::new();
        if let Some(_order_book) = order_book {
            params.insert("order_book".to_owned(), _order_book.to_string());
        }
        let url = String::from("https://api.bitso.com/v3/order_book/");
        self.get(&url, &mut params).await
    }

    /// Make a get request to pull a specific trade
    /// https://bitso.com/api_info/#trades
    pub async fn get_order_trades(
        &self,
        book: Option<&str>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut params = HashMap::new();
        if let Some(_book) = book {
            params.insert("book".to_owned(), _book.to_string());
        }
        let url = String::from("https://api.bitso.com/v3/trades/");
        self.get(&url, &mut params).await
    }
}

