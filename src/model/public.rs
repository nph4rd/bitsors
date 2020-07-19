/// From: https://bitso.com/api_info#available-books
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AvailableBook {
    book: Option<String>,
    minimum_amount: Option<String>,
    maximum_amount: Option<String>,
    minimum_price: Option<String>,
    maximum_price: Option<String>,
    minimum_value: Option<String>,
    maximum_value: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BookTicker {
    book: Option<String>,
    volume: Option<String>,
    high: Option<String>,
    last: Option<String>,
    low: Option<String>,
    vwap: Option<String>,
    ask: Option<String>,
    bid: Option<String>,
    created_at: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OrderBookPayload {
    asks: Vec<Ask>,
    bids: Vec<Bid>,
    updated_at: Option<String>,
    sequence: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Ask {
    book: Option<String>,
    price: Option<String>,
    amount: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Bid {
    book: Option<String>,
    price: Option<String>,
    amount: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Trade {
    book: Option<String>,
    created_at: Option<String>,
    amount: Option<String>,
    maker_side: Option<String>,
    price: Option<String>,
    tid: Option<u64>,
}
