extern crate bitsors;
use bitsors::websocket::*;
fn main() {
    let mut socket = BitsoWebSocket::new().unwrap();
    //subscribe to the BTC-MXN orders channel
    socket
        .subscribe(Subscription::Orders, Books::BtcMxn)
        .unwrap();
    // You can iterate over the Books and Subscription channels
    for book in Books::iter() {
        for subs in Subscription::iter() {
            socket.subscribe(subs, book).unwrap();
        }
    }
    loop {
        match socket.read().unwrap() {
            Response::Orders(r) => println!("{:?}", r),
            Response::Trades(r) => println!("{:?}", r),
            Response::DiffOrders(r) => println!("{:?}", r),
        }
    }
}
