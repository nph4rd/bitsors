
/// From: https://bitso.com/api_info#account-status
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AccountStatus {
    pub success: bool,
    pub payload: AccountStatusPayload,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AccountStatusPayload {
    client_id: String,
    first_name: String,
    last_name: String,
    status: String,
    daily_limit: String,
    daily_remaining: String,
    monthly_limit: String,
    monthly_remaining: String,
    cash_deposit_allowance: String,
    cellphone_number: String,
    cellphone_number_stored: String,
    email_stored: String,
    official_id: String,
    proof_of_residency: String,
    signed_contract: String,
    origin_of_funds: String,
}
