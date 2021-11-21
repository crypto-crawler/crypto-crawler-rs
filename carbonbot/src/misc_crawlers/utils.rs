use std::sync::mpsc::Sender;

use crypto_crawler::Message;
use crypto_market_type::MarketType;
use crypto_msg_type::MessageType;

// create a thread to convert Sender<Message> Sender<String>
pub(super) fn create_conversion_thread(
    exchange: String,
    msg_type: MessageType,
    market_type: MarketType,
    tx: Sender<Message>,
) -> Sender<String> {
    let (tx_raw, rx_raw) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        for json in rx_raw {
            let msg = Message::new(exchange.clone(), market_type, msg_type, json);
            tx.send(msg).unwrap();
        }
    });
    tx_raw
}
