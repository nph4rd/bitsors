use std::collections::HashMap;
use reqwest::Client;
use reqwest::Method;

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
}

