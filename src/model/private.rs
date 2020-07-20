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
    tid: Option<u64>,
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
    beneficiary_name: Option<String>,
    beneficiary_bank: Option<String>,
    beneficiary_clabe: Option<String>,
    numeric_reference: Option<String>,
    concepto: Option<String>,
    clave_rastreo: Option<String>,
    cep: Option<Cep>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Cep {
    r#return: Option<CepReturn>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CepReturn {
    cda: Option<Cda>,
    estado_consulta: Option<String>,
    url: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cda {
    cadena_original: Option<String>,
    concepto_pago: Option<String>,
    cuenta_beneficiario: Option<String>,
    cuenta_ordenante: Option<String>,
    fecha_captura: Option<String>,
    fecha_operacion: Option<String>,
    hora: Option<String>,
    iva: Option<String>,
    monto: Option<String>,
    nombre_beneficiario: Option<String>,
    nombre_inst_beneficiaria: Option<String>,
    nombre_inst_ordenante: Option<String>,
    nombre_ordenante: Option<String>,
    sello_digital: Option<String>,
    serie_certificado: Option<String>,
    tipo_operacion: Option<String>,
    tipo_pago: Option<String>,
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
    sender_clabe: Option<String>,
    receive_clabe: Option<String>,
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
    tid: Option<u64>,
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
    tid: Option<u64>,
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
    unfilled_amount: Option<String>,
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Withdrawal<T> {
    pub wid: Option<String>,
    pub status: Option<String>,
    pub created_at: Option<String>,
    pub currency: Option<String>,
    pub method: Option<String>,
    pub amount: Option<String>,
    pub details: T,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CryptoWithdrawal {
    withdrawal_address: Option<String>,
    tx_hash: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SPEIWithdrawal {
    sender_name: Option<String>,
    receive_clabe: Option<String>,
    sender_clabe: Option<String>,
    numeric_reference: Option<String>,
    concepto: Option<String>,
    clave_rastreo: Option<String>,
    beneficiary_name: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DebitWithdrawal {
    sender_name: Option<String>,
    receive_clabe: Option<String>,
    sender_clabe: Option<String>,
    numeric_reference: Option<String>,
    concepto: Option<String>,
    clave_rastreo: Option<String>,
    beneficiary_name: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PhoneWithdrawal {
    sender_name: Option<String>,
    receive_clabe: Option<String>,
    sender_clabe: Option<String>,
    numeric_reference: Option<String>,
    concepto: Option<String>,
    clave_rastreo: Option<String>,
    beneficiary_name: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BankCode {
    code: Option<String>,
    name: Option<String>,
}
