use super::super::util::deserialize_books;
use super::super::websocket::Books;

// ------------------------------- Trades -------------------------------------
#[derive(Debug, Clone, PartialEq, serde_derive::Deserialize)]
pub struct Trades {
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(deserialize_with = "deserialize_books")]
    pub book: Books,
    pub payload: Vec<TradesPayload>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Deserialize)]
pub struct TradesPayload {
    /// A unique number identifying the transaction
    pub i: i64,
    /// Amount
    pub a: String,
    /// Rate
    pub r: String,
    /// Value
    pub v: String,
}

// ------------------------------- DiffOrders -------------------------------------

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct DiffOrders {
    #[serde(rename = "type")]
    pub type_field: String,
    pub book: String,
    pub payload: Vec<DiffOrdersPayload>,
    pub sequence: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct DiffOrdersPayload {
    /// Unix timestamp
    pub d: u64,
    /// Rate
    pub r: String,
    /// 0 indicates buy 1 indicates sell
    pub t: u8,
    /// Order ID
    pub o: String,
    pub s: String,
}

// ------------------------------- Orders -------------------------------------

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Deserialize)]
pub struct Orders {
    #[serde(rename = "type")]
    pub type_field: String,
    pub book: String,
    pub payload: OrdersPayload,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Deserialize)]
pub struct OrdersPayload {
    pub bids: Vec<BidAsk>,
    pub asks: Vec<BidAsk>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Deserialize)]
pub struct BidAsk {
    /// Rate
    pub r: f64,
    /// Amount
    pub a: f64,
    /// Value
    pub v: f64,
    /// 0 indicates buy 1 indicates sell
    pub t: u8,
    /// Unix timestamp
    pub d: u64,
}
