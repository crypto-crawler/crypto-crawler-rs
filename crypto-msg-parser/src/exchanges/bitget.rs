use crypto_market_type::MarketType;

use crate::{MessageType, TradeMsg, TradeSide};

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use std::collections::HashMap;

const EXCHANGE_NAME: &str = "bitget";

lazy_static! {
    // See https://bitgetlimited.github.io/apidoc/en/swap/#contract-information
    static ref LINEAR_SWAP_MAPPING: HashMap<&'static str, f64> = vec![
        ("BTC", 0.001),
        ("EOS", 1.0),
        ("ETH", 0.1),
        ("LTC", 0.1),
        ("XRP", 10.0),
        ("BCH", 0.01),
        ("ETC", 1.0),
        ("ADA", 100.0),
        ("LINK", 1.0),
        ("TRX", 100.0),
        ("DOT", 1.0),
        ("XTZ", 1.0),
        ("UNI", 1.0),
        ("SUSHI", 1.0),
        ("YFI", 0.0001),
        ("ATOM", 1.0),
        ("FIL", 0.1),
        ("ALGO", 10.0),
        ("COMP", 0.01),
    ].into_iter().collect();
}

// see https://bitgetlimited.github.io/apidoc/en/swap/#public-trading-channel
#[derive(Serialize, Deserialize)]
struct SwapTradeMsg {
    instrument_id: String,
    price: String,
    side: String, // buy, sell
    size: String,
    timestamp: String,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
struct WebsocketMsg<T: Sized> {
    table: String,
    data: Vec<T>,
}

fn calc_quantity_and_volume(
    market_type: MarketType,
    pair: &str,
    price: f64,
    quantity: f64,
) -> (f64, f64) {
    if market_type == MarketType::LinearSwap {
        let base = {
            let slash_pos = pair.find('/').unwrap();
            &pair[..slash_pos]
        };
        let contract_value = LINEAR_SWAP_MAPPING.get(base).unwrap();

        let real_quantity = contract_value * quantity;
        (real_quantity, real_quantity * price)
    } else if market_type == MarketType::InverseSwap {
        let volume = quantity;
        (volume / price, volume)
    } else {
        panic!("Unknown market_type {}", market_type);
    }
}

pub(crate) fn parse_trade(market_type: MarketType, msg: &str) -> Result<Vec<TradeMsg>> {
    let ws_msg = serde_json::from_str::<WebsocketMsg<SwapTradeMsg>>(&msg)?;
    let trades: Vec<TradeMsg> = ws_msg
        .data
        .into_iter()
        .map(|raw_trade| {
            let mut trade = TradeMsg {
                exchange: EXCHANGE_NAME.to_string(),
                market_type,
                symbol: raw_trade.instrument_id.clone(),
                pair: crypto_pair::normalize_pair(&raw_trade.instrument_id, EXCHANGE_NAME).unwrap(),
                msg_type: MessageType::Trade,
                timestamp: raw_trade.timestamp.parse::<i64>().unwrap(),
                price: raw_trade.price.parse::<f64>().unwrap(),
                quantity: raw_trade.size.parse::<f64>().unwrap(),
                volume: 0.0,
                side: if raw_trade.side == "sell" {
                    TradeSide::Sell
                } else {
                    TradeSide::Buy
                },
                // Use timestamp as ID because bitget doesn't provide trade_id
                trade_id: raw_trade.timestamp.to_string(),
                raw: serde_json::to_value(raw_trade).unwrap(),
            };
            let (quantity, volume) =
                calc_quantity_and_volume(market_type, &trade.pair, trade.price, trade.quantity);
            trade.quantity = quantity;
            trade.volume = volume;
            trade
        })
        .collect();

    Ok(trades)
}
