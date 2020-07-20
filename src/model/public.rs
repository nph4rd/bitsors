#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AvailableBook {
    pub book: Option<String>,
    pub minimum_amount: Option<String>,
    pub maximum_amount: Option<String>,
    pub minimum_price: Option<String>,
    pub maximum_price: Option<String>,
    pub minimum_value: Option<String>,
    pub maximum_value: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BookTicker {
    pub book: Option<String>,
    pub volume: Option<String>,
    pub high: Option<String>,
    pub last: Option<String>,
    pub low: Option<String>,
    pub vwap: Option<String>,
    pub ask: Option<String>,
    pub bid: Option<String>,
    pub created_at: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OrderBookPayload {
    pub asks: Vec<Ask>,
    pub bids: Vec<Bid>,
    pub updated_at: Option<String>,
    pub sequence: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Ask {
    pub book: Option<String>,
    pub price: Option<String>,
    pub amount: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Bid {
    pub book: Option<String>,
    pub price: Option<String>,
    pub amount: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Trade {
    pub book: Option<String>,
    pub created_at: Option<String>,
    pub amount: Option<String>,
    pub maker_side: Option<String>,
    pub price: Option<String>,
    pub tid: Option<u64>,
}
