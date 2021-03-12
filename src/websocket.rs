use std::str::FromStr;

pub use strum::{EnumCount, IntoEnumIterator};

use serde::{de, Deserialize, Deserializer};
use strum_macros::{AsRefStr, Display, EnumCount, EnumIter, EnumString};
use tungstenite::{client::AutoStream, connect, Message, WebSocket};

/// Bitso WebSocket
/// See: <https://bitso.com/api_info?#websocket-api>
///
/// Check all the possible options in [`Books`] and [`Subscription`]
///
/// # Examples
/// ```no_run
/// use bitsors::websocket::*;
///
/// let mut socket = BitsoWebSocket::new();
///
/// // subscribe to the BTC-MXN orders channel
/// socket.subscribe(Subscription::Orders, Books::BtcMxn);
///                                                         
/// // You can iterate over the Books and Subscription channels
/// for book in Books::iter() {
///     for subs in Subscription::iter() {
///         socket.subscribe(subs, book);
///     }
/// }
///
/// loop {
///     match socket.read() {
///         Response::Orders(r) => println!("{:?}", r),
///         Response::Trades(r) => println!("{:?}", r),
///         Response::DiffOrders(r) => println!("{:?}", r),
///     }
/// }
/// ```

pub struct BitsoWebSocket {
    socket: WebSocket<AutoStream>,
}

impl BitsoWebSocket {
    pub fn new() -> Self {
        let (socket, _) = connect("wss://ws.bitso.com").expect("Can't connect");
        BitsoWebSocket { socket }
    }

    pub fn close(&mut self) {
        self.socket.close(None).unwrap();
    }

    pub fn subscribe(&mut self, subscription_type: Subscription, book: Books) {
        let request = format!(
            r#"{{"action":"subscribe","book":"{}","type":"{}"}}"#,
            book.as_ref(),
            subscription_type.as_ref()
        );
        self.socket.write_message(Message::Text(request)).unwrap();
        let subscription_response = self.socket.read_message().unwrap().into_text().unwrap();
        if !subscription_response.contains("ok") {
            panic!("bad response from server");
        }
    }

    pub fn read(&mut self) -> Response {
        let mut data = self.socket.read_message().unwrap().into_text().unwrap();
        while data.contains(r#""type":"ka""#) || data.contains("subscribe") {
            //ignore keep alive and subscribe messages
            data = self.socket.read_message().unwrap().into_text().unwrap();
        }

        let first_comma = data.find(':').unwrap();
        let second_comma = data[first_comma + 1..].find(',').unwrap() + first_comma;

        match Subscription::from_str(&data[first_comma + 2..second_comma]).unwrap() {
            Subscription::Trades => Response::Trades(serde_json::from_str(&data).unwrap()),

            Subscription::DiffOrders => Response::DiffOrders(serde_json::from_str(&data).unwrap()),

            Subscription::Orders => Response::Orders(serde_json::from_str(&data).unwrap()),
        }
    }
}

impl Default for BitsoWebSocket {
    fn default() -> Self {
        Self::new()
    }
}

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

/// Represents the possible subscription responses in the Bitso API.
#[derive(Debug, Clone, PartialEq, Display, AsRefStr, EnumCount)]
pub enum Response {
    ///[Trades Channel](https://bitso.com/api_info?#trades-channel)
    Trades(Trades),

    ///[Diff-Orders](https://bitso.com/api_info?#diff-orders)
    DiffOrders(DiffOrders),

    ///[Orders](https://bitso.com/api_info?#orders)
    Orders(Orders),
}

/// Represents the three subscription channels in the Bitso API.
#[derive(Debug, Copy, Clone, PartialEq, Display, AsRefStr, EnumString, EnumIter)]
pub enum Subscription {
    ///[Trades Channel](https://bitso.com/api_info?#trades-channel)
    #[strum(serialize = "trades")]
    Trades,

    ///[Diff-Orders](https://bitso.com/api_info?#diff-orders)
    #[strum(serialize = "diff-orders")]
    DiffOrders,

    ///[Orders](https://bitso.com/api_info?#orders)
    #[strum(serialize = "orders")]
    Orders,
}

/// Represents all the possible exchanges available in Bitso.
/// See: <https://bitso.com/api_info#available-books>
#[derive(Debug, Copy, Clone, PartialEq, Display, AsRefStr, EnumCount, EnumIter, EnumString)]
pub enum Books {
    #[strum(serialize = "btc_mxn")]
    BtcMxn,
    #[strum(serialize = "eth_btc")]
    EthBtc,
    #[strum(serialize = "eth_ars")]
    EthArs,
    #[strum(serialize = "eth_mxn")]
    EthMxn,
    #[strum(serialize = "xrp_btc")]
    XrpBtc,
    #[strum(serialize = "xrp_mxn")]
    XrpMxn,
    #[strum(serialize = "ltc_btc")]
    LtcBtc,
    #[strum(serialize = "ltc_mxn")]
    LtcMxn,
    #[strum(serialize = "bch_btc")]
    BchBtc,
    #[strum(serialize = "bch_mxn")]
    BchMxn,
    #[strum(serialize = "tusd_btc")]
    TusdBtc,
    #[strum(serialize = "tusd_mxn")]
    TusdMxn,
    #[strum(serialize = "mana_btc")]
    ManaBtc,
    #[strum(serialize = "mana_mxn")]
    ManaMxn,
    #[strum(serialize = "bat_btc")]
    BatBtc,
    #[strum(serialize = "bat_mxn")]
    BatMxn,
    #[strum(serialize = "btc_ars")]
    BtcArs,
    #[strum(serialize = "btc_dai")]
    BtcDai,
    #[strum(serialize = "dai_mxn")]
    DaiMxn,
    #[strum(serialize = "btc_usd")]
    BtcUsd,
    #[strum(serialize = "xrp_usd")]
    XrpUsd,
    #[strum(serialize = "eth_usd")]
    EthUsd,
    #[strum(serialize = "dai_ars")]
    DaiArs,
    #[strum(serialize = "btc_brl")]
    BtcBrl,
}

//custom deserialize implementation for Books
fn deserialize_books<'de, D>(deserializer: D) -> Result<Books, D::Error>
where
    D: Deserializer<'de>,
{
    let book = <&str>::deserialize(deserializer)?;
    Books::from_str(book).map_err(de::Error::custom)
}
