use std::collections::HashMap;

/// Bitso API object
pub struct Bitso {
    pub prefix: String,
}

impl Bitso {
    pub fn default() -> Bitso {
        Bitso {
            prefix: "https://api.bitso.com/v3/".to_owned(),
        }
    }
    pub fn prefix(mut self, prefix: &str) -> Bitso {
        self.prefix = prefix.to_owned();
        self
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
            let resp = reqwest::get(url).await?;
            let body = resp.text().await?;
                // .json::<HashMap<String, String>>()
                // .await?;
            println!("{:?}", body);
            Ok(())
        }
    }
}

