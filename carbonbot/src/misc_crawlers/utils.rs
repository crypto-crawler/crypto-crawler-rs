use std::sync::mpsc::Sender;

use crypto_crawler::{MarketType, Message, MessageType};

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
