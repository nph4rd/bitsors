/// From: https://bitso.com/api_info#available-books
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AvailableBooks {
    pub success: bool,
    pub payload: Vec<AvailableBook>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AvailableBook {
    book: String,
    minimum_amount: String,
    maximum_amount: String,
    minimum_price: String,
    maximum_price: String,
    minimum_value: String,
    maximum_value: String,
}

/// From: https://bitso.com/api_info#ticker
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Ticker {
    pub success: bool,
    pub payload: BookTicker,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BookTicker {
    book: String,
    volume: String,
    high: String,
    last: String,
    low: String,
    vwap: String,
    ask: String,
    bid: String,
    created_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OrderBook {
    pub success: bool,
    pub payload: OrderBookPayload,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OrderBookPayload {
    asks: Vec<Ask>,
    bids: Vec<Bid>,
    updated_at: String,
    sequence: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Ask {
    book: String,
    price: String,
    amount: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Bid {
    book: String,
    price: String,
    amount: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Trades {
    pub test: bool,
}
