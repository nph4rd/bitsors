use std::env;
use dotenv::dotenv;

/// Client credentials object for Bitso
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BitsoCredentials {
    pub api_key: String,
    pub api_secret: String
}

impl BitsoCredentials {

    /// Create BitsoCredentials object
    pub fn default() -> BitsoCredentials {
        dotenv().ok();
        let api_key = env::var("API_KEY")
            .unwrap_or_default();
        let api_secret = env::var("API_SECRET")
            .unwrap_or_default();
        BitsoCredentials {
            api_key,
            api_secret,
        }
    }

    /// Set api_key
    pub fn api_key(
        mut self,
        api_key: &str,
    ) -> BitsoCredentials {
        self.api_key = api_key.to_owned();
        self
    }

    /// Set api_secret
    pub fn api_secret(
        mut self,
        api_secret: &str,
    ) -> BitsoCredentials {
        self.api_secret = api_secret.to_owned();
        self
    }

    /// Build BitsoCredentials object
    pub fn build(self) -> BitsoCredentials {
        self
    }

    /// Get api_key
    pub fn get_key(&self) -> String {
        self.api_key.to_owned()
    }

    /// Get api_secret
    pub fn get_secret(&self) -> String {
        self.api_secret.to_owned()
    }
}

