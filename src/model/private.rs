
/// From: https://bitso.com/api_info#account-status
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AccountStatus {
    pub client_id: String,
    pub firts_name: String,
    pub last_name: String,
    pub status: String,
    pub daily_limit: String,
    pub daily_remaining: String,
    pub monthly_limit: String,
    pub monthly_remaining: String,
    pub cash_deposit_allowance: String,
    pub cellphone_number: String,
    pub cellphone_number_stored: String,
    pub email_stored: String,
    pub official_id: String,
    pub proof_of_residency: String,
    pub signed_contract: String,
    pub origin_of_funds: String,
}
