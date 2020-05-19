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
    pub fn default() -> Bitso {
        Bitso {  }
    }
    pub fn build(self) -> Bitso {
        self
    }
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
    pub async fn get_available_books(
        &self
    ) -> Result<(), Box<dyn std::error::Error>> {
        let url = String::from("https://api.bitso.com/v3/available_books/");
        self.get(&url, &mut HashMap::new()).await
    }
    pub async fn get_ticker(
        &self
    ) -> Result<(), Box<dyn std::error::Error>> {
        let url = String::from("https://api.bitso.com/v3/ticker/");
        self.get(&url, &mut HashMap::new()).await
    }
    pub async fn get_order_book(
        &self,
        order_book: Option<&str>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut params = HashMap::new();
        if let Some(_order_book) = order_book {
            params.insert("order_book".to_owned(), _order_book.to_string());
        }
        let url = String::from("https://api.bitso.com/v3/ticker/");
        self.get(&url, &mut params).await
    }
}

