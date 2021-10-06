use crypto_market_type::MarketType;

use crate::exchanges::utils::{calc_quantity_and_volume, http_get};
use crate::Order;
use crate::{FundingRateMsg, MessageType, OrderBookMsg, TradeMsg, TradeSide};

use chrono::prelude::*;
use chrono::DateTime;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use std::collections::{BTreeMap, BTreeSet, HashMap};

const EXCHANGE_NAME: &str = "bitmex";

lazy_static! {
    // symbol -> tickSize
    static ref SYMBOL_INDEX_AND_TICK_SIZE_MAP: HashMap<String, (usize, f64)> = {
        let mut m: HashMap<String, (usize, f64)> = vec![
            ("AAVEUSDT", (89, 0.01)),
            ("ADAUSD", (176, 0.0001)),
            ("ADAUSDT", (21, 0.00001)),
            ("ADAZ21", (138, 0.00000001)),
            ("ALTMEXUSD", (116, 0.01)),
            ("AVAXUSD", (175, 0.001)),
            ("AXSUSDT", (125, 0.001)),
            ("BCHUSD", (402, 0.05)),
            ("BCHZ21", (137, 0.000001)),
            ("BNBUSD", (178, 0.01)),
            ("BNBUSDT", (42, 0.01)),
            ("DEFIMEXUSD", (117, 0.01)),
            ("DOGEUSD", (177, 0.00001)),
            ("DOGEUSDT", (476, 0.00001)),
            ("DOTUSD", (179, 0.001)),
            ("DOTUSDT", (19, 0.0005)),
            ("EOSUSDT", (39, 0.0005)),
            ("EOSZ21", (139, 0.00000001)),
            ("ETHUSD", (297, 0.05)),
            ("ETHUSDZ21", (141, 0.05)),
            ("ETHZ21", (134, 0.00001)),
            ("FILUSDT", (56, 0.01)),
            ("LINKUSDT", (441, 0.0005)),
            ("LTCUSD", (407, 0.01)),
            ("LTCZ21", (135, 0.000001)),
            ("LUNAUSD", (149, 0.001)),
            ("MATICUSDT", (88, 0.0001)),
            ("SOLUSDT", (49, 0.001)),
            ("SRMUSDT", (132, 0.001)),
            ("SUSHIUSDT", (118, 0.001)),
            ("TRXUSDT", (40, 0.00001)),
            ("TRXZ21", (140, 0.0000000001)),
            ("UNIUSDT", (20, 0.001)),
            ("VETUSDT", (81, 0.00001)),
            ("XBTEUR", (64, 0.5)),
            ("XBTEURZ21", (142, 0.5)),
            ("XBTH22", (133, 0.5)),
            ("XBTUSD", (88, 0.01)),
            ("XBTV21", (150, 0.5)),
            ("XBTZ21", (65, 0.5)),
            ("XLMUSDT", (22, 0.00001)),
            ("XRPUSD", (377, 0.0001)),
            ("XRPZ21", (136, 0.00000001)),
        ]
        .into_iter()
        .map(|x| (x.0.to_string(), x.1))
        .collect();

        let from_online = fetch_tick_sizes();
        for (symbol, tick_size) in from_online {
            m.insert(symbol, tick_size);
        }

        m
    };
}

fn fetch_active_symbols() -> BTreeSet<String> {
    let mut active_symbols: BTreeSet<String> = vec![
        "AAVEUSDT",
        "ADAUSD",
        "ADAUSDT",
        "ADAZ21",
        "ALTMEXUSD",
        "AVAXUSD",
        "AXSUSDT",
        "BCHUSD",
        "BCHZ21",
        "BNBUSD",
        "BNBUSDT",
        "DEFIMEXUSD",
        "DOGEUSD",
        "DOGEUSDT",
        "DOTUSD",
        "DOTUSDT",
        "EOSUSDT",
        "EOSZ21",
        "ETHUSD",
        "ETHUSDZ21",
        "ETHZ21",
        "FILUSDT",
        "LINKUSDT",
        "LTCUSD",
        "LTCZ21",
        "LUNAUSD",
        "MATICUSDT",
        "SOLUSDT",
        "SRMUSDT",
        "SUSHIUSDT",
        "TRXUSDT",
        "TRXZ21",
        "UNIUSDT",
        "VETUSDT",
        "XBTEUR",
        "XBTEURZ21",
        "XBTH22",
        "XBTUSD",
        "XBTV21",
        "XBTZ21",
        "XLMUSDT",
        "XRPUSD",
        "XRPZ21",
    ]
    .into_iter()
    .map(|x| (x.to_string()))
    .collect();
    if let Ok(txt) = http_get("https://www.bitmex.com/api/v1/instrument/active") {
        if let Ok(instruments) = serde_json::from_str::<Vec<HashMap<String, Value>>>(&txt) {
            for instrument in instruments {
                let symbol = instrument
                    .get("symbol")
                    .unwrap()
                    .as_str()
                    .unwrap()
                    .to_string();
                active_symbols.insert(symbol);
            }
        }
    }
    active_symbols
}

fn fetch_tick_sizes() -> BTreeMap<String, (usize, f64)> {
    #[derive(Serialize, Deserialize)]
    #[allow(non_snake_case)]
    struct TickSize {
        symbol: String,
        timestamp: String,
        tickSize: f64,
    }
    let active_symbols = fetch_active_symbols();
    let mut m: BTreeMap<String, (usize, f64)> = BTreeMap::new();
    let mut start = 0_usize;
    loop {
        let url = format!(
            "https://www.bitmex.com/api/v1/instrument?columns=symbol,tickSize&start={}&count=500",
            start
        );
        if let Ok(txt) = http_get(url.as_str()) {
            if let Ok(tick_sizes) = serde_json::from_str::<Vec<TickSize>>(&txt) {
                let n = tick_sizes.len();
                for (index, tick_size) in tick_sizes.into_iter().enumerate() {
                    if active_symbols.contains(&tick_size.symbol) {
                        let real_tick_size = if tick_size.symbol == "XBTUSD" {
                            0.01 // legacy reason, see https://www.bitmex.com/app/wsAPI#OrderBookL2
                        } else {
                            tick_size.tickSize
                        };
                        m.insert(tick_size.symbol, (index, real_tick_size));
                    }
                }
                if n < 500 {
                    break;
                } else {
                    start += 500;
                }
            }
        }
    }
    assert_eq!(active_symbols.len(), m.len());
    m
}

// see https://www.bitmex.com/app/wsAPI#Response-Format
#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct RawTradeMsg {
    timestamp: String,
    symbol: String,
    side: String, // Sell, Buy'
    size: f64,
    price: f64,
    tickDirection: String, // MinusTick, PlusTick, ZeroMinusTick, ZeroPlusTick
    trdMatchID: String,
    grossValue: f64,
    homeNotional: f64,
    foreignNotional: f64,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct RawOrder {
    symbol: String,
    id: usize,
    side: String, // Sell, Buy
    size: Option<f64>,
    price: Option<f64>,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct RawFundingRateMsg {
    timestamp: String,
    symbol: String,
    fundingInterval: String,
    fundingRate: f64,
    fundingRateDaily: f64,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
struct WebsocketMsg<T: Sized> {
    table: String,
    action: String,
    data: Vec<T>,
}

pub(crate) fn extract_symbol(_market_type: MarketType, msg: &str) -> Option<String> {
    let ws_msg = serde_json::from_str::<WebsocketMsg<Value>>(msg).unwrap();
    let symbols = ws_msg
        .data
        .iter()
        .map(|v| v["symbol"].as_str().unwrap())
        .collect::<Vec<&str>>();
    if symbols.is_empty() {
        None
    } else {
        Some(symbols[0].to_string())
    }
}

// Copied from crypto-markets/tests/bitmex.rs
fn get_market_type_from_symbol(symbol: &str) -> MarketType {
    let date = &symbol[(symbol.len() - 2)..];
    if date.parse::<i64>().is_ok() {
        // future
        if symbol.starts_with("XBT") {
            // Settled in XBT, quoted in USD
            MarketType::InverseFuture
        } else if (&symbol[..(symbol.len() - 3)]).ends_with("USD") {
            // Settled in XBT, quoted in USD
            MarketType::QuantoFuture
        } else {
            // Settled in XBT, quoted in XBT
            MarketType::LinearFuture
        }
    } else {
        // swap
        if symbol.starts_with("XBT") {
            // Settled in XBT, quoted in USD
            MarketType::InverseSwap
        } else {
            MarketType::QuantoSwap
        }
    }
}

pub(crate) fn parse_trade(market_type: MarketType, msg: &str) -> Result<Vec<TradeMsg>> {
    let ws_msg = serde_json::from_str::<WebsocketMsg<RawTradeMsg>>(msg)?;
    let raw_trades = ws_msg.data;
    let mut trades: Vec<TradeMsg> = raw_trades
        .into_iter()
        .map(|raw_trade| {
            // assert_eq!(raw_trade.foreignNotional, raw_trade.homeNotional * raw_trade.price); // tiny diff actually exists
            let timestamp = DateTime::parse_from_rfc3339(&raw_trade.timestamp).unwrap();
            let market_type = if market_type == MarketType::Unknown {
                get_market_type_from_symbol(&raw_trade.symbol)
            } else {
                market_type
            };
            TradeMsg {
                exchange: EXCHANGE_NAME.to_string(),
                market_type,
                symbol: raw_trade.symbol.to_string(),
                pair: crypto_pair::normalize_pair(&raw_trade.symbol, EXCHANGE_NAME).unwrap(),
                msg_type: MessageType::Trade,
                timestamp: timestamp.timestamp_millis(),
                price: raw_trade.price,
                quantity_base: raw_trade.homeNotional,
                quantity_quote: raw_trade.foreignNotional,
                quantity_contract: Some(raw_trade.size),
                side: if raw_trade.side == "Sell" {
                    TradeSide::Sell
                } else {
                    TradeSide::Buy
                },
                trade_id: raw_trade.trdMatchID.clone(),
                json: serde_json::to_string(&raw_trade).unwrap(),
            }
        })
        .collect();
    if trades.len() == 1 {
        trades[0].json = msg.to_string();
    }
    Ok(trades)
}

pub(crate) fn parse_funding_rate(
    market_type: MarketType,
    msg: &str,
) -> Result<Vec<FundingRateMsg>> {
    let ws_msg = serde_json::from_str::<WebsocketMsg<RawFundingRateMsg>>(msg)?;
    let mut rates: Vec<FundingRateMsg> = ws_msg
        .data
        .into_iter()
        .map(|raw_msg| {
            let settlement_time = DateTime::parse_from_rfc3339(&raw_msg.timestamp).unwrap();
            let market_type = if market_type == MarketType::Unknown {
                get_market_type_from_symbol(&raw_msg.symbol)
            } else {
                market_type
            };
            FundingRateMsg {
                exchange: EXCHANGE_NAME.to_string(),
                market_type,
                symbol: raw_msg.symbol.clone(),
                pair: crypto_pair::normalize_pair(&raw_msg.symbol, EXCHANGE_NAME).unwrap(),
                msg_type: MessageType::FundingRate,
                timestamp: Utc::now().timestamp_millis(),
                funding_rate: raw_msg.fundingRate,
                funding_time: settlement_time.timestamp_millis(),
                estimated_rate: None,
                json: serde_json::to_string(&raw_msg).unwrap(),
            }
        })
        .collect();
    if rates.len() == 1 {
        rates[0].json = msg.to_string();
    }
    Ok(rates)
}

// convert ID to price
// https://www.bitmex.com/app/wsAPI#OrderBookL2
// price = (100000000 * symbolIdx - ID) * tickSize
fn price_from_id(symbol: &str, id: usize) -> f64 {
    let (index, tick_size) = SYMBOL_INDEX_AND_TICK_SIZE_MAP.get(symbol).unwrap();
    ((100000000 * index - id) as f64) * tick_size
}

pub(crate) fn parse_l2(
    market_type: MarketType,
    msg: &str,
    timestamp: i64,
) -> Result<Vec<OrderBookMsg>> {
    let ws_msg = serde_json::from_str::<WebsocketMsg<RawOrder>>(msg)?;
    let snapshot = ws_msg.action == "partial";
    if ws_msg.data.is_empty() {
        return Ok(Vec::new());
    }
    let symbol = ws_msg.data[0].symbol.clone();
    let pair = crypto_pair::normalize_pair(&symbol, EXCHANGE_NAME).unwrap();
    let market_type = if market_type == MarketType::Unknown {
        get_market_type_from_symbol(&symbol)
    } else {
        market_type
    };

    let parse_order = |raw_order: &RawOrder| -> Order {
        let price = if let Some(p) = raw_order.price {
            p
        } else {
            price_from_id(&raw_order.symbol, raw_order.id)
        };

        let quantity = raw_order.size.unwrap_or(0.0); // 0.0 means delete
        let (quantity_base, quantity_quote, quantity_contract) =
            calc_quantity_and_volume(EXCHANGE_NAME, market_type, &pair, price, quantity);
        Order {
            price,
            quantity_base,
            quantity_quote,
            quantity_contract,
        }
    };

    let orderbook = OrderBookMsg {
        exchange: EXCHANGE_NAME.to_string(),
        market_type,
        symbol,
        pair: pair.clone(),
        msg_type: MessageType::L2Event,
        timestamp,
        asks: ws_msg
            .data
            .iter()
            .filter(|x| x.side == "Sell")
            .map(|x| parse_order(x))
            .collect(),
        bids: ws_msg
            .data
            .iter()
            .filter(|x| x.side == "Buy")
            .map(|x| parse_order(x))
            .collect(),
        snapshot,
        json: msg.to_string(),
    };

    Ok(vec![orderbook])
}

#[cfg(test)]
mod tests {
    #[test]
    #[ignore]
    fn test_fetch_active_symbols() {
        let active_symbols = super::fetch_active_symbols();
        assert!(active_symbols.len() > 0);
        for symbol in active_symbols {
            println!("\"{}\",", symbol);
        }
    }

    #[test]
    #[ignore]
    fn test_fetch_tick_sizes() {
        let tick_sizes = super::fetch_tick_sizes();
        assert!(tick_sizes.len() > 0);
        for (symbol, tick_size) in tick_sizes {
            println!("(\"{}\", ({}, {})),", symbol, tick_size.0, tick_size.1);
        }
    }

    #[test]
    fn test_price_from_id() {
        // data are from https://www.bitmex.com/api/v1/orderBook/L2?symbol=XBTUSD&depth=25
        assert_eq!(51366.5, super::price_from_id("XBTUSD", 8794863350));
        assert_eq!(51306.0, super::price_from_id("XBTUSD", 8794869400));

        assert_eq!(3460.0, super::price_from_id("ETHUSD", 29699930800));
        assert_eq!(3451.0, super::price_from_id("ETHUSD", 29699930980));
    }
}
