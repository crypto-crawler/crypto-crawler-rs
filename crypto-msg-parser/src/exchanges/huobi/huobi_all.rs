use crypto_market_type::MarketType;
use crypto_msg_type::MessageType;
use serde::{Serialize, Deserialize};
use simple_error::SimpleError;

use crate::{BboMsg, exchanges::{huobi::{message::WebsocketMsg, EXCHANGE_NAME}, utils::calc_quantity_and_volume}, KlineMsg};

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct RawBboMsgStop {
    seqId: u64,
    ask: f64,
    askSize: f64,
    bid: f64,
    bidSize: f64,
    quoteTime: u64,
    symbol: String
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct RawBboMsgInverseSwap {
    mrid: u64,
    id: u64,
    bid: [f64;2],
    ask: [f64;2],
    ts: u64,
    version: u64,
    ch: String
}


pub(super) fn parse_bbo_stop(
    market_type: MarketType,
    msg: &str,
    received_at: Option<i64>,
) -> Result<BboMsg, SimpleError> {

    let ws_msg = serde_json::from_str::<WebsocketMsg<RawBboMsgStop>>(msg).map_err(|_e| {
        SimpleError::new(format!(
            "Failed to deserialize {} to WebsocketMsg<RawBboMsg>",
            msg
        ))
    })?;
    debug_assert!(ws_msg.ch.ends_with("bbo"));
    let timestamp = if market_type == MarketType::Spot {
        received_at.unwrap()
    } else {
        ws_msg.ts
    };

    let symbol = ws_msg.tick.symbol;
    let pair = crypto_pair::normalize_pair(symbol.as_str(), EXCHANGE_NAME).unwrap();

    let (ask_quantity_base, ask_quantity_quote, ask_quantity_contract) = calc_quantity_and_volume(
        EXCHANGE_NAME,
        market_type,
        &pair,
        ws_msg.tick.ask,
        ws_msg.tick.askSize
    );

    let (bid_quantity_base, bid_quantity_quote, bid_quantity_contract) = calc_quantity_and_volume(
        EXCHANGE_NAME,
        market_type,
        &pair,
        ws_msg.tick.bid,
        ws_msg.tick.bidSize
    );

    let bbo_msg = BboMsg {
        exchange: EXCHANGE_NAME.to_string(),
        market_type,
        symbol: symbol.to_string(),
        pair,
        msg_type: MessageType::BBO,
        timestamp,
        ask_price: ws_msg.tick.ask,
        ask_quantity_base,
        ask_quantity_quote,
        ask_quantity_contract,
        bid_price: ws_msg.tick.bid,
        bid_quantity_base,
        bid_quantity_quote,
        bid_quantity_contract,
        id: Some(ws_msg.tick.seqId),
        json: msg.to_string(),
    };

    Ok(bbo_msg)
}


pub(super) fn parse_bbo(
    market_type: MarketType,
    msg: &str,
    _received_at: Option<i64>
) -> Result<BboMsg, SimpleError> {
    let ws_msg = serde_json::from_str::<WebsocketMsg<RawBboMsgInverseSwap>>(msg).map_err(|_e| {
        SimpleError::new(format!(
            "Failed to deserialize {} to WebsocketMsg<RawBboMsg>",
            msg
        ))
    })?;

    debug_assert!(ws_msg.ch.ends_with("bbo"));
    let timestamp = ws_msg.ts;

    let symbol = ws_msg.tick.ch.split(".").nth(1).unwrap();
    let pair = crypto_pair::normalize_pair(symbol, EXCHANGE_NAME).unwrap();

    let (ask_quantity_base, ask_quantity_quote, ask_quantity_contract) = calc_quantity_and_volume(
        EXCHANGE_NAME,
        market_type,
        &pair,
        ws_msg.tick.ask[0],
        ws_msg.tick.ask[1]
    );

    let (bid_quantity_base, bid_quantity_quote, bid_quantity_contract) = calc_quantity_and_volume(
        EXCHANGE_NAME,
        market_type,
        &pair,
        ws_msg.tick.bid[0],
        ws_msg.tick.bid[1]
    );

    let bbo_msg = BboMsg {
        exchange: EXCHANGE_NAME.to_string(),
        market_type,
        symbol: symbol.to_string(),
        pair,
        msg_type: MessageType::BBO,
        timestamp,
        ask_price: ws_msg.tick.ask[0],
        ask_quantity_base,
        ask_quantity_quote,
        ask_quantity_contract,
        bid_price: ws_msg.tick.bid[0],
        bid_quantity_base,
        bid_quantity_quote,
        bid_quantity_contract,
        id: Some(ws_msg.tick.id),
        json: msg.to_string(),
    };

    Ok(bbo_msg)

}


// {
//     "ch":"market.btcusdt.kline.1min",
//     "ts":1655978055321,
//     "tick":{
//         "id":1655978040,
//         "open":20674.12,
//         "close":20685.42,
//         "low":20668.7,
//         "high":20685.42,
//         "amount":3.1574597233321917,
//         "vol":65290.57880845,
//         "count":73
//     }
// }

#[derive(Serialize, Deserialize)]
struct RawKlineMsg {
    id: u64,
    open: f64,
    close: f64,
    low: f64,
    high: f64,
    amount: f64,
    vol: f64,
    count: u64
}


pub(super) fn parse_candlestick(
    market_type: MarketType,
    msg: &str,
    msg_type: MessageType
) -> Result<KlineMsg, SimpleError> {
    let obj = serde_json::from_str::<WebsocketMsg<RawKlineMsg>>(msg).map_err(|_e| {
        SimpleError::new(format!(
            "Failed to deserialize {} to HashMap<String, Value>",
            msg
        ))
    })?;

    let ch: Vec<&str>= obj.ch.split(".").collect();
    let symbol = ch[1].to_string();
    let period = ch[3].to_string();


    // 1min, 5min, 15min, 30min, 60min, 4hour, 1day, 1mon, 1week, 1year
    let len = period.len();

    let period = match &period[len-3..] {
        "min" => format!("{}m", &period[..len-3]),
        "mon" => format!("{}M", &period[..len-3]),
        "day" => format!("{}D", &period[..len-3]),
        _ => if len > 3 {
            match &period[len-4..] {
                "year" => format!("{}Y", &period[..len-4]),
                "week" => format!("{}W", &period[..len-4]),
                "hour" => format!("{}H", &period[..len-4]),
                _ => format!("{}N", &period[..len-3])
            }
        } else {
            format!("{}N", &period[len-3..])
        }
    };


    let pair = crypto_pair::normalize_pair(&symbol, EXCHANGE_NAME).unwrap();

    // obj.data.k

    let kline_msg = KlineMsg {
        exchange: EXCHANGE_NAME.to_string(),
        market_type,
        symbol,
        pair,
        msg_type,
        timestamp: obj.ts,
        json: msg.to_string(),
        open: obj.tick.open,
        high: obj.tick.high,
        low: obj.tick.low,
        close: obj.tick.close,
        volume: obj.tick.vol,
        period,
        quote_volume: None
    };

    Ok(kline_msg)
}