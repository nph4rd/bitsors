#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AccountStatusPayload {
    client_id: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
    status: Option<String>,
    daily_limit: Option<String>,
    daily_remaining: Option<String>,
    monthly_limit: Option<String>,
    monthly_remaining: Option<String>,
    cash_deposit_allowance: Option<String>,
    cellphone_number: Option<String>,
    cellphone_number_stored: Option<String>,
    email_stored: Option<String>,
    official_id: Option<String>,
    proof_of_residency: Option<String>,
    signed_contract: Option<String>,
    origin_of_funds: Option<String>,
}

/// From: https://bitso.com/api_info#account-balance
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AccountBalance {
    pub success: bool,
    pub payload: Balances,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Balances {
    balances: Vec<AccountBalanceInstance>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AccountBalanceInstance {
    currency: Option<String>,
    available: Option<String>,
    locked: Option<String>,
    total: Option<String>,
    pending_deposit: Option<String>,
    pending_withdrawal: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FeesPayload {
    fees: Vec<BookFee>,
    withdrawal_fees: WithdrawalFees,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BookFee {
    book: Option<String>,
    taker_fee_decimal: Option<String>,
    taker_fee_percent: Option<String>,
    maker_fee_decimal: Option<String>,
    maker_fee_percent: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WithdrawalFees {
    btc: Option<String>,
    eth: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LedgerInstance {
    eid: Option<String>,
    operation: Option<String>,
    created_at: Option<String>,
    balance_updates: Vec<BalanceUpdate>,
    details: BalanceDetails,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BalanceUpdate {
    currency: Option<String>,
    amount: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BalanceDetails {
    tid: Option<String>,
    oid: Option<String>,
    fid: Option<String>,
    wid: Option<String>,
    method: Option<String>,
    method_name: Option<String>,
    funding_address: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WithdrawalsPayload {
    wid: Option<String>,
    status: Option<String>,
    created_at: Option<String>,
    currency: Option<String>,
    method: Option<String>,
    amount: Option<String>,
    details: WithdrawalDetails,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WithdrawalDetails {
    withdrawal_address: Option<String>,
    tx_hash: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FundingsPayload {
    fid: Option<String>,
    status: Option<String>,
    created_at: Option<String>,
    currency: Option<String>,
    method: Option<String>,
    amount: Option<String>,
    details: FundingDetails,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FundingDetails {
    funding_address: Option<String>,
    tx_hash: Option<String>,
    sender_name: Option<String>,
    sender_bank: Option<String>,
    sender_clave: Option<String>,
    receive_clave: Option<String>,
    numeric_reference: Option<String>,
    concepto: Option<String>,
    clave_rastreo: Option<String>,
    beneficiary_name: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserTradesPayload {
    book: Option<String>,
    major: Option<String>,
    created_at: Option<String>,
    minor: Option<String>,
    fees_amount: Option<String>,
    fees_currency: Option<String>,
    price: Option<String>,
    tid: Option<String>,
    oid: Option<String>,
    side: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OrderTradesPayload {
    book: Option<String>,
    major: Option<String>,
    created_at: Option<String>,
    minor: Option<String>,
    fees_amount: Option<String>,
    fees_currency: Option<String>,
    price: Option<String>,
    tid: Option<String>,
    oid: Option<String>,
    client_id: Option<String>,
    side: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OpenOrdersPayload {
    book: Option<String>,
    original_amount: Option<String>,
    unfilled_amount: Option<String>,
    original_value: Option<String>,
    created_at: Option<String>,
    updated_at: Option<String>,
    price: Option<String>,
    oid: Option<String>,
    client_id: Option<String>,
    side: Option<String>,
    status: Option<String>,
    r#type: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LookupOrdersPayload {
    book: Option<String>,
    original_amount: Option<String>,
    unfilled_amount: Option<u64>,
    original_value: Option<String>,
    created_at: Option<String>,
    updated_at: Option<String>,
    price: Option<String>,
    oid: Option<String>,
    side: Option<String>,
    status: Option<String>,
    r#type: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlaceOrderPayload {
    oid: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FundingDestination {
    pub account_identifier_name: Option<String>,
    pub account_identifier: Option<String>,
}
