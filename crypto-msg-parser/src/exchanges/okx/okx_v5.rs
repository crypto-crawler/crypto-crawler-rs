use crypto_market_type::MarketType;
use crypto_msg_type::MessageType;

use super::super::utils::calc_quantity_and_volume;
use crypto_message::{KlineMsg, BboMsg, FundingRateMsg, Order, OrderBookMsg, TradeMsg, TradeSide};

use super::EXCHANGE_NAME;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use simple_error::SimpleError;
use std::{collections::HashMap};

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

// https://www.okx.com/docs-v5/en/#rest-api-market-data-get-ticker
// {
// 	"arg": {
// 		"channel": "tickers",
// 		"instId": "BTC-USD-SWAP"
// 	},
// 	"data": [{
// 		"instType": "SWAP",
// 		"instId": "BTC-USD-SWAP",
// 		"last": "31771.6",
// 		"lastSz": "16",
// 		"askPx": "31771.6",
// 		"askSz": "16",
// 		"bidPx": "31771.5",
// 		"bidSz": "1967",
// 		"open24h": "31648.1",
// 		"high24h": "32398.1",
// 		"low24h": "31202.4",
// 		"sodUtc0": "31717.3",
// 		"sodUtc8": "32038.6",
// 		"volCcy24h": "13760.6923",
// 		"vol24h": "4364424",
// 		"ts": "1654033212805"
// 	}]
// }
#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct RawBboSwapMsg {
    instType: String,
    instId: String,
    last: String,
    lastSz: String,
    askPx: String,
    askSz: String,
    bidPx: String,
    bidSz: String,
    open24h: String,
    high24h: String,
    low24h: String,
    sodUtc0: String,
    sodUtc8: String,
    volCcy24h: String,
    vol24h: String,
    ts: String,
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
struct WebBboMsg<T: Sized> {
    arg: Arg,
    data: Vec<T>,
}

#[derive(Serialize, Deserialize)]
struct RestfulMsg<T: Sized> {
    code: String,
    msg: String,
    data: Vec<T>,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

/// https://www.okx.com/docs-v5/en/#websocket-api-public-channel-candlesticks-channel
// {
// 	"arg": {
// 		"channel": "candle1m",
// 		"instId": "BTC-USDT"
// 	},
// 	"data": [
// 		["1654154580000", "29930.7", "29936.3", "29930.7", "29936.3", "0.0111536", "333.86246417"]
// 	]
// }
#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct RawKlineMsg {
    ts: String,         // Data generation time, Unix timestamp format in milliseconds, e.g. 1597026383085
    open: String,       // Open price
    high: String,       // highest price
    low: String,        // Lowest price
    close: String,      // Close price
    vol: String,        // Trading volume, with a unit of contact.
    volCcy: String,     // Trading volume, with a unit of currency.
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

pub(crate) fn parse_l2_topk(
    market_type: MarketType,
    msg: &str,
) -> Result<Vec<OrderBookMsg>, SimpleError> {
    parse_l2(market_type, msg)
}

pub(crate) fn parse_bbo(
    market_type: MarketType,
    msg: &str,
) -> Result<BboMsg, SimpleError> {
    if market_type == MarketType::InverseSwap || market_type == MarketType::LinearSwap{
        parse_bbo_swap(market_type, msg)
    } else if market_type == MarketType::Spot || market_type == MarketType::InverseFuture || market_type == MarketType::LinearFuture {
        parse_bbo_book(market_type, msg)
    } else {
        Err(SimpleError::new("Not implemented"))
    }
}
pub(crate) fn parse_bbo_swap(
    market_type: MarketType,
    msg: &str,
) -> Result<BboMsg, SimpleError> {
    let mut ws_msg =  serde_json::from_str::<WebBboMsg<RawBboSwapMsg>>(msg).map_err(|_e| {
        SimpleError::new(format!(
            "Failed to deserialize {} to WebBboMsg<RawBboSwapMsg>",
            msg
        ))
    })?;
  
    let symbol = &ws_msg.arg.instId.as_str();

    let bbo_msg_vec = ws_msg.data.get_mut(0).unwrap();
    let timestamp = bbo_msg_vec.ts.parse::<i64>().unwrap();
    let pair = crypto_pair::normalize_pair(symbol, EXCHANGE_NAME).unwrap();

    let price =  bbo_msg_vec.askPx.as_str().parse::<f64>().unwrap();
    let quantity = bbo_msg_vec.askSz.as_str().parse::<f64>().unwrap();
    let ask_price = price.clone();

    let (ask_quantity_base, ask_quantity_quote, ask_quantity_contract) = calc_quantity_and_volume(
        EXCHANGE_NAME,
        market_type,
        &pair,
        price,
        quantity,
    );

    let price =  bbo_msg_vec.bidPx.as_str().parse::<f64>().unwrap();
    let quantity = bbo_msg_vec.bidSz.as_str().parse::<f64>().unwrap();
    let bid_price = price.clone();

    let (bid_quantity_base, bid_quantity_quote, bid_quantity_contract) = calc_quantity_and_volume(
        EXCHANGE_NAME,
        market_type,
        &pair,
        price,
        quantity,
    );

    let bbo_msg = BboMsg {
        exchange: EXCHANGE_NAME.to_string(),
        market_type,
        symbol: symbol.to_string(),
        pair,
        msg_type: MessageType::BBO,
        timestamp,
        ask_price,
        ask_quantity_base,
        ask_quantity_quote,
        ask_quantity_contract,
        bid_price,
        bid_quantity_base,
        bid_quantity_quote,
        bid_quantity_contract,
        id: None,
        json: msg.to_string(),
    };

    Ok(bbo_msg) 
}

pub(crate) fn parse_bbo_book(
    market_type: MarketType,
    msg: &str,
) -> Result<BboMsg, SimpleError> {
    let mut ws_msg =  serde_json::from_str::<WebBboMsg<RawOrderbookMsg>>(msg).map_err(|_e| {
        SimpleError::new(format!(
            "Failed to deserialize {} to WebBboMsg<RawOrderbookMsg>",
            msg
        ))
    })?;
    
    debug_assert_eq!(ws_msg.data.len(), 1);

    let symbol = &ws_msg.arg.instId.as_str();
    let bbo_msg_vec = ws_msg.data.get_mut(0).unwrap();
    let timestamp = bbo_msg_vec.ts.parse::<i64>().unwrap();
    let pair = crypto_pair::normalize_pair(symbol, EXCHANGE_NAME).unwrap();
    // Order book on sell side
    let price =  bbo_msg_vec.asks[0][0].parse::<f64>().unwrap();
    let quantity = bbo_msg_vec.asks[0][1].parse::<f64>().unwrap();
    let ask_price = price.clone();

    let (ask_quantity_base, ask_quantity_quote, ask_quantity_contract) = calc_quantity_and_volume(
        EXCHANGE_NAME,
        market_type,
        &pair,
        price,
        quantity,
    );
    // Order book on buy side
    let price = bbo_msg_vec.bids[0][0].parse::<f64>().unwrap();
    let quantity = bbo_msg_vec.bids[0][1].parse::<f64>().unwrap();
    let bid_price = price.clone();

    let (bid_quantity_base, bid_quantity_quote, bid_quantity_contract) = calc_quantity_and_volume(
        EXCHANGE_NAME,
        market_type,
        &pair,
        price,
        quantity,
    );

    let bbo_msg = BboMsg {
        exchange: EXCHANGE_NAME.to_string(),
        market_type,
        symbol: symbol.to_string(),
        pair,
        msg_type: MessageType::BBO,
        timestamp,
        ask_price,
        ask_quantity_base,
        ask_quantity_quote,
        ask_quantity_contract,
        bid_price,
        bid_quantity_base,
        bid_quantity_quote,
        bid_quantity_contract,
        id: None,
        json: msg.to_string(),
    };

    Ok(bbo_msg) 
}

pub(crate) fn parse_candlestick(
    market_type: MarketType,
    msg: &str,
    message_type: MessageType
) -> Result<KlineMsg, SimpleError> {
    if market_type == MarketType::EuropeanOption {
        Err(SimpleError::new("Not implemented"))
    } else if market_type == MarketType::InverseSwap {
        swap_candlestick(market_type, msg, message_type)
    } else {
        kline_candlestick(market_type, msg, message_type)
    }
}

pub(super) fn swap_candlestick(
    market_type: MarketType,
    msg: &str,
    msg_type: MessageType
) -> Result<KlineMsg, SimpleError> {
    let mut obj = serde_json::from_str::<WebBboMsg<RawBboSwapMsg>>(msg).map_err(|_e| {
        SimpleError::new(format!(
            "Failed to deserialize {} to WebBboMsg<RawBboSwapMsg>",
            msg
        ))
    })?;

    let symbol = obj.arg.instId.to_string();
    // candle1Y
    // candle6M candle3M candle1M
    // candle1W
    // candle1D candle2D candle3D candle5D
    // candle12H candle6H candle4H candle2H candle1H
    // candle30m candle15m candle5m candle3m candle1m
    let s_period = obj.arg.channel.to_string().replace("utc", "");
    let period = &s_period[6..s_period.len()];
    // 1m, 3m, 5m, 15m, 30m, 1H, 1D, 1M, 1W, 1Y
    let len = period.len();

    let period = match &period[len-1..] {
        "m" => format!("{}m", &period[..len-1]),
        "M" => format!("{}M", &period[..len-1]),
        "D" => format!("{}D", &period[..len-1]),
        "Y" => format!("{}Y", &period[..len-1]),
        "W" => format!("{}W", &period[..len-1]),
        "H" => format!("{}H", &period[..len-1]),
        _ => format!("{}N", &period[..len-1])
    };


    let pair = crypto_pair::normalize_pair(&symbol, EXCHANGE_NAME).unwrap();
    let ojb_data = obj.data.get_mut(0).unwrap();

    let kline_msg = KlineMsg {
        exchange: EXCHANGE_NAME.to_string(),
        market_type,
        symbol,
        pair,
        msg_type,
        timestamp: ojb_data.ts.parse::<i64>().unwrap(),
        json: msg.to_string(),
        open: ojb_data.open24h.parse::<f64>().unwrap(),
        high: ojb_data.high24h.parse::<f64>().unwrap(),
        low: ojb_data.low24h.parse::<f64>().unwrap(),
        close: ojb_data.volCcy24h.parse::<f64>().unwrap(),
        volume: ojb_data.vol24h.parse::<f64>().unwrap(),
        period,
        quote_volume: None,
    };

    Ok(kline_msg)
}

pub(super) fn kline_candlestick(
    market_type: MarketType,
    msg: &str,
    msg_type: MessageType
) -> Result<KlineMsg, SimpleError> {
    let mut obj = serde_json::from_str::<WebBboMsg<RawKlineMsg>>(msg).map_err(|_e| {
        SimpleError::new(format!(
            "Failed to deserialize {} to WebBboMsg<RawKlineMsg>",
            msg
        ))
    })?;

    let symbol = obj.arg.instId.to_string();
    // candle1Y
    // candle6M candle3M candle1M
    // candle1W
    // candle1D candle2D candle3D candle5D
    // candle12H candle6H candle4H candle2H candle1H
    // candle30m candle15m candle5m candle3m candle1m
    let s_period = obj.arg.channel.to_string().replace("utc", "");
    let period = &s_period[6..s_period.len()];
    // 1m, 3m, 5m, 15m, 30m, 1H, 1D, 1M, 1W, 1Y
    let len = period.len();

    let period = match &period[len-1..] {
        "m" => format!("{}m", &period[..len-1]),
        "M" => format!("{}M", &period[..len-1]),
        "D" => format!("{}D", &period[..len-1]),
        "Y" => format!("{}Y", &period[..len-1]),
        "W" => format!("{}W", &period[..len-1]),
        "H" => format!("{}H", &period[..len-1]),
        _ => format!("{}N", &period[..len-1])
    };


    let pair = crypto_pair::normalize_pair(&symbol, EXCHANGE_NAME).unwrap();
    let ojb_data = obj.data.get_mut(0).unwrap();

    let kline_msg = KlineMsg {
        exchange: EXCHANGE_NAME.to_string(),
        market_type,
        symbol,
        pair,
        msg_type,
        timestamp: ojb_data.ts.parse::<i64>().unwrap(),
        json: msg.to_string(),
        open: ojb_data.open.parse::<f64>().unwrap(),
        high: ojb_data.high.parse::<f64>().unwrap(),
        low: ojb_data.low.parse::<f64>().unwrap(),
        close: ojb_data.close.parse::<f64>().unwrap(),
        volume: ojb_data.vol.parse::<f64>().unwrap(),
        period,
        quote_volume: None,
    };

    Ok(kline_msg)
}
