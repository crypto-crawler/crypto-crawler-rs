use crypto_market_type::MarketType;
use crypto_msg_type::MessageType;

use super::super::utils::calc_quantity_and_volume;
use crypto_message::{FundingRateMsg, Order, OrderBookMsg, TradeMsg, TradeSide};

use super::EXCHANGE_NAME;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use simple_error::SimpleError;
use std::collections::HashMap;

// https://www.okx.com/docs-v5/en/#websocket-api-public-channel-trades-channel
#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct RawTradeMsg {
    instId: String,
    tradeId: String,
    px: String,
    sz: String,
    side: String,
    ts: String,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

// https://www.okx.com/docs-v5/en/#websocket-api-public-channel-order-book-channel
#[derive(Serialize, Deserialize)]
struct RawOrderbookMsg {
    asks: Vec<[String; 4]>,
    bids: Vec<[String; 4]>,
    ts: String,
    checksum: Option<i64>,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

// https://www.okx.com/docs-v5/en/#websocket-api-public-channel-funding-rate-channel
#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct RawFundingRateMsg {
    instType: String,
    instId: String,
    fundingRate: String,
    nextFundingRate: String,
    fundingTime: String,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct Arg {
    channel: String,
    instId: String,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
struct WebsocketMsg<T: Sized> {
    arg: Arg,
    action: Option<String>, // snapshot, update, only applicable to order book
    data: Vec<T>,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
struct RestfulMsg<T: Sized> {
    code: String,
    msg: String,
    data: Vec<T>,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

pub(super) fn extract_symbol(msg: &str) -> Result<String, SimpleError> {
    if let Ok(ws_msg) = serde_json::from_str::<WebsocketMsg<Value>>(msg) {
        Ok(ws_msg.arg.instId)
    } else if let Ok(rest_msg) = serde_json::from_str::<RestfulMsg<HashMap<String, Value>>>(msg) {
        if rest_msg.code != "0" {
            return Err(SimpleError::new(format!("Error HTTP response {}", msg)));
        }
        #[allow(clippy::comparison_chain)]
        if rest_msg.data.len() > 1 {
            Ok("ALL".to_string())
        } else if rest_msg.data.len() == 1 {
            let first_elem = &rest_msg.data[0];
            if let Some(inst_id) = first_elem.get("instId") {
                Ok(inst_id.as_str().unwrap().to_string())
            } else {
                Ok("NONE".to_string())
            }
        } else {
            Ok("NONE".to_string())
        }
    } else {
        Err(SimpleError::new(format!(
            "Unsupported message format {}",
            msg
        )))
    }
}

pub(super) fn extract_timestamp(msg: &str) -> Result<Option<i64>, SimpleError> {
    if let Ok(ws_msg) = serde_json::from_str::<WebsocketMsg<Value>>(msg) {
        if ws_msg.arg.channel == "funding-rate" {
            return Ok(None);
        }
        let channel = ws_msg.arg.channel.as_str();
        let timestamp = ws_msg
            .data
            .iter()
            .map(|x| {
                (if channel.starts_with("candle") {
                    x[0].as_str().unwrap()
                } else {
                    x["ts"].as_str().unwrap()
                })
                .parse::<i64>()
                .unwrap()
            })
            .max();

        if timestamp.is_none() {
            Err(SimpleError::new(format!("data is empty in {}", msg)))
        } else {
            Ok(timestamp)
        }
    } else if let Ok(rest_msg) = serde_json::from_str::<RestfulMsg<HashMap<String, Value>>>(msg) {
        if rest_msg.code != "0" {
            return Err(SimpleError::new(format!("Error HTTP response {}", msg)));
        }
        let timestamp = rest_msg
            .data
            .iter()
            .map(|obj| obj["ts"].as_str().unwrap().parse::<i64>().unwrap())
            .max();
        Ok(timestamp)
    } else {
        Err(SimpleError::new(format!(
            "Unsupported message format {}",
            msg
        )))
    }
}

pub(super) fn get_msg_type(msg: &str) -> MessageType {
    if let Ok(ws_msg) = serde_json::from_str::<WebsocketMsg<Value>>(msg) {
        let channel = ws_msg.arg.channel.as_str();
        match channel {
            "trades" => MessageType::Trade,
            "books" | "books-l2-tbt" | "books50-l2-tbt" => MessageType::L2Event,
            "books5" => MessageType::L2TopK,
            "bbo-tbt" => MessageType::BBO,
            "tickers" => MessageType::Ticker,
            "funding-rate" => MessageType::FundingRate,
            _ => {
                if channel.starts_with("candle") {
                    MessageType::Candlestick
                } else {
                    MessageType::Other
                }
            }
        }
    } else {
        MessageType::Other
    }
}

pub(super) fn parse_trade(
    market_type: MarketType,
    msg: &str,
) -> Result<Vec<TradeMsg>, SimpleError> {
    let ws_msg = serde_json::from_str::<WebsocketMsg<RawTradeMsg>>(msg).map_err(|_e| {
        SimpleError::new(format!(
            "Failed to deserialize {} to WebsocketMsg<RawTradeMsg>",
            msg
        ))
    })?;

    let mut trades: Vec<Result<TradeMsg, SimpleError>> = ws_msg
        .data
        .into_iter()
        .map(|raw_trade| {
            let timestamp = raw_trade.ts.parse::<i64>().unwrap();
            let price = raw_trade.px.parse::<f64>().unwrap();
            let size = raw_trade.sz.parse::<f64>().unwrap();
            let pair = crypto_pair::normalize_pair(&raw_trade.instId, EXCHANGE_NAME).unwrap();
            let (quantity_base, quantity_quote, _) =
                calc_quantity_and_volume(EXCHANGE_NAME, market_type, &pair, price, size);

            Ok(TradeMsg {
                exchange: EXCHANGE_NAME.to_string(),
                market_type,
                symbol: raw_trade.instId.clone(),
                pair,
                msg_type: MessageType::Trade,
                timestamp,
                price,
                quantity_base,
                quantity_quote,
                quantity_contract: if market_type == MarketType::Spot {
                    None
                } else {
                    Some(size)
                },
                side: if raw_trade.side == "sell" {
                    TradeSide::Sell
                } else {
                    TradeSide::Buy
                },
                trade_id: raw_trade.tradeId.clone(),
                json: serde_json::to_string(&raw_trade).unwrap(),
            })
        })
        .collect();

    if trades.len() == 1 {
        if let Ok(v) = trades[0].as_mut() {
            v.json = msg.to_string();
        }
    }
    trades.into_iter().collect()
}

pub(super) fn parse_funding_rate(
    market_type: MarketType,
    msg: &str,
    received_at: i64,
) -> Result<Vec<FundingRateMsg>, SimpleError> {
    let ws_msg = serde_json::from_str::<WebsocketMsg<RawFundingRateMsg>>(msg).map_err(|_e| {
        SimpleError::new(format!(
            "Failed to deserialize {} to WebsocketMsg<RawFundingRateMsg>",
            msg
        ))
    })?;

    let mut rates: Vec<FundingRateMsg> = ws_msg
        .data
        .into_iter()
        .map(|raw_msg| {
            let pair = crypto_pair::normalize_pair(&raw_msg.instId, EXCHANGE_NAME).unwrap();
            FundingRateMsg {
                exchange: EXCHANGE_NAME.to_string(),
                market_type,
                symbol: raw_msg.instId.clone(),
                pair,
                msg_type: MessageType::FundingRate,
                timestamp: received_at,
                funding_rate: raw_msg.fundingRate.parse::<f64>().unwrap(),
                funding_time: raw_msg.fundingTime.parse::<i64>().unwrap(),
                estimated_rate: Some(raw_msg.nextFundingRate.parse::<f64>().unwrap()),
                json: serde_json::to_string(&raw_msg).unwrap(),
            }
        })
        .collect();

    if rates.len() == 1 {
        rates[0].json = msg.to_string();
    }
    Ok(rates)
}

pub(super) fn parse_l2(
    market_type: MarketType,
    msg: &str,
) -> Result<Vec<OrderBookMsg>, SimpleError> {
    let ws_msg = serde_json::from_str::<WebsocketMsg<RawOrderbookMsg>>(msg).map_err(|_e| {
        SimpleError::new(format!(
            "Failed to deserialize {} to WebsocketMsg<RawOrderbookMsg>",
            msg
        ))
    })?;

    let channel = ws_msg.arg.channel.as_str();
    let msg_type = if channel == "books5" {
        MessageType::L2TopK
    } else {
        MessageType::L2Event
    };
    let snapshot = {
        if let Some(action) = ws_msg.action {
            action == "snapshot"
        } else {
            channel == "books5"
        }
    };
    debug_assert_eq!(ws_msg.data.len(), 1);

    let symbol = ws_msg.arg.instId.as_str();
    let pair = crypto_pair::normalize_pair(symbol, EXCHANGE_NAME).unwrap();

    let mut orderbooks = ws_msg
        .data
        .iter()
        .map(|raw_orderbook| {
            let timestamp = raw_orderbook.ts.parse::<i64>().unwrap();
            let parse_order = |raw_order: &[String; 4]| -> Order {
                let price = raw_order[0].parse::<f64>().unwrap();
                let quantity = raw_order[1].parse::<f64>().unwrap();
                let (quantity_base, quantity_quote, quantity_contract) =
                    calc_quantity_and_volume(EXCHANGE_NAME, market_type, &pair, price, quantity);

                Order {
                    price,
                    quantity_base,
                    quantity_quote,
                    quantity_contract,
                }
            };

            OrderBookMsg {
                exchange: EXCHANGE_NAME.to_string(),
                market_type,
                symbol: symbol.to_string(),
                pair: pair.clone(),
                msg_type,
                timestamp,
                seq_id: None,
                prev_seq_id: None,
                asks: raw_orderbook
                    .asks
                    .iter()
                    .map(|x| parse_order(x))
                    .collect::<Vec<Order>>(),
                bids: raw_orderbook
                    .bids
                    .iter()
                    .map(|x| parse_order(x))
                    .collect::<Vec<Order>>(),
                snapshot,
                json: serde_json::to_string(raw_orderbook).unwrap(),
            }
        })
        .collect::<Vec<OrderBookMsg>>();

    if orderbooks.len() == 1 {
        orderbooks[0].json = msg.to_string();
    }
    Ok(orderbooks)
}
