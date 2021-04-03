use super::model::websocket::*;
use std::str::FromStr;

pub use strum::{EnumCount, IntoEnumIterator};

use strum_macros::{AsRefStr, Display, EnumCount, EnumIter, EnumString};
use tungstenite::{client::AutoStream, connect, error::Result, Message, WebSocket};

/// Bitso WebSocket object.
///
/// For more info see: <https://bitso.com/api_info?#websocket-api>
///
/// Check all the possible options in [`Books`] and [`Subscription`]
///
/// # Examples
/// ```no_run
/// use bitsors::websocket::*;
///
/// let mut socket = BitsoWebSocket::new().unwrap();
///
/// // You can subscribe to a specific orders channel
/// socket.subscribe(Subscription::Orders, Books::BtcMxn).unwrap();
///                                                         
/// // You can also iterate over all the Books and Subscription channels
/// for book in Books::iter() {
///     for subs in Subscription::iter() {
///         socket.subscribe(subs, book).unwrap();
///     }
/// }
///
/// loop {
///     match socket.read().unwrap() {
///         Response::Orders(r) => println!("{:?}", r),
///         Response::Trades(r) => println!("{:?}", r),
///         Response::DiffOrders(r) => println!("{:?}", r),
///     }
/// }
/// ```

#[derive(Debug)]
pub struct BitsoWebSocket {
    socket: WebSocket<AutoStream>,
}

impl BitsoWebSocket {
    /// Creates a new WebSocket connection.
    pub fn new() -> Result<Self> {
        let (socket, _) = connect("wss://ws.bitso.com")?;
        Ok(BitsoWebSocket { socket })
    }

    /// Closes an existing WebSocket connection.
    pub fn close(&mut self) -> Result<()> {
        self.socket.close(None)
    }

    /// Creates a subscription request to a given channel.
    pub fn subscribe(&mut self, subscription_type: Subscription, book: Books) -> Result<String> {
        let request = format!(
            r#"{{"action":"subscribe","book":"{}","type":"{}"}}"#,
            book.as_ref(),
            subscription_type.as_ref()
        );
        self.socket.write_message(Message::Text(request))?;
        self.socket.read_message()?.into_text()
    }

    /// Reads the response from the WebSocket connection.
    pub fn read(&mut self) -> Result<Response> {
        let mut data = self.socket.read_message()?.into_text()?;
        while data.contains(r#""type":"ka""#) || data.contains("subscribe") {
            //ignore keep alive and subscribe messages
            data = self.socket.read_message()?.into_text()?;
        }
        let first_comma = data.find(':').unwrap();
        let second_comma = data[first_comma + 1..].find(',').unwrap() + first_comma;
        match Subscription::from_str(&data[first_comma + 2..second_comma]).unwrap() {
            Subscription::Trades => Ok(Response::Trades(serde_json::from_str(&data).unwrap())),
            Subscription::DiffOrders => {
                Ok(Response::DiffOrders(serde_json::from_str(&data).unwrap()))
            }
            Subscription::Orders => Ok(Response::Orders(serde_json::from_str(&data).unwrap())),
        }
    }
}

/// Represents the possible subscription responses in the Bitso WebSocket API.
#[derive(Debug, Clone, PartialEq, Display, AsRefStr, EnumCount)]
pub enum Response {
    ///[Trades Channel](https://bitso.com/api_info?#trades-channel)
    Trades(Trades),

    ///[Diff-Orders](https://bitso.com/api_info?#diff-orders)
    DiffOrders(DiffOrders),

    ///[Orders](https://bitso.com/api_info?#orders)
    Orders(Orders),
}

/// Represents the three subscription channels in the Bitso WebSocket API.
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

/// Represents all the possible books available in Bitso.
///
/// For more info, see: <https://bitso.com/api_info#available-books>
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
