
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
    currency: String,
    available: String,
    locked: String,
    total: String,
    pending_deposit: String,
    pending_withdrawal: String,
}

/// From: https://bitso.com/api_info#account-balance
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Fees {
    pub success: bool,
    pub payload: FeesPayload,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FeesPayload {
    fees: Vec<BookFee>,
    withdrawal_fees: WithdrawalFees,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BookFee {
    book: String,
    taker_fee_decimal: String,
    taker_fee_percent: String,
    maker_fee_decimal: String,
    maker_fee_percent: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WithdrawalFees {
    btc: String,
    eth: String,
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Ledger {
    pub success: bool,
    pub payload: Vec<LedgerInstance>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LedgerInstance {
    eid: String,
    operation: String,
    created_at: String,
    balance_updates: Vec<BalanceUpdate>,
    details: BalanceDetails,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BalanceUpdate {
    currency: String,
    amount: String,
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
pub struct Withdrawals {
    pub success: bool,
    pub payload: Vec<WithdrawalsPayload>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WithdrawalsPayload {
    wid: String,
    status: String,
    created_at: String,
    currency: String,
    method: String,
    amount: String,
    details: WithdrawalDetails,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WithdrawalDetails {
    withdrawal_address: String,
    tx_hash: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Fundings {
    pub success: bool,
    pub payload: Vec<FundingsPayload>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FundingsPayload {
    fid: String,
    status: String,
    created_at: String,
    currency: String,
    method: String,
    amount: String,
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
pub struct UserTrades {
    success: bool,
    payload: Vec<UserTradesPayload>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserTradesPayload {
    book: String,
    major: String,
    created_at: String,
    minor: String,
    fees_amount: String,
    fees_currency: String,
    price: String,
    tid: String,
    oid: String,
    side: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OrderTrades {
    success: bool,
    payload: Vec<OrderTradesPayload>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OrderTradesPayload {
    book: String,
    major: String,
    created_at: String,
    minor: String,
    fees_amount: String,
    fees_currency: String,
    price: String,
    tid: String,
    oid: String,
    client_id: String,
    side: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OpenOrders {
    pub success: bool,
    pub payload: Vec<OpenOrdersPayload>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OpenOrdersPayload {
    book: String,
    original_amount: String,
    unfilled_amount: String,
    original_value: String,
    created_at: String,
    updated_at: String,
    price: String,
    oid: String,
    client_id: String,
    side: String,
    status: String,
    r#type: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LookupOrders {
    pub success: bool,
    pub payload: Vec<LookupOrdersPayload>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LookupOrdersPayload {
    book: String,
    original_amount: String,
    unfilled_amount: String,
    original_value: String,
    created_at: String,
    updated_at: String,
    price: String,
    oid: String,
    side: String,
    status: String,
    r#type: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OrderCancellation {
    pub success: bool,
    pub payload: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlaceOrder {
    pub success: bool,
    pub payload: PlaceOrderPayload,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlaceOrderPayload {
    oid: String,
}
