use std::env;
use dotenv::dotenv;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BitsoCredentials {
    pub api_key: String,
    pub api_secret: String
}

impl BitsoCredentials {
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
}

