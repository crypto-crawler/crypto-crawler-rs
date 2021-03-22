use crypto_market_type::MarketType;

use super::super::utils::http_get;
use crate::{MessageType, TradeMsg, TradeSide};

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use std::collections::HashMap;

const EXCHANGE_NAME: &str = "zbg";

lazy_static! {
    static ref SWAP_CONTRACT_MAP: HashMap<i64, SwapContractInfo> = fetch_swap_contracts();
}

struct SwapContractInfo {
    symbol: String,
    contract_id: i64,
    contract_unit: f64,
}

// See https://zbgapi.github.io/docs/future/v1/en/#public-get-contracts
fn fetch_swap_contracts() -> HashMap<i64, SwapContractInfo> {
    #[derive(Serialize, Deserialize)]
    #[allow(non_snake_case)]
    struct SwapMarket {
        symbol: String,
        currencyName: String,
        lotSize: String,
        contractId: i64,
        takerFeeRatio: String,
        commodityId: i64,
        currencyId: i64,
        contractUnit: String,
        makerFeeRatio: String,
        priceTick: String,
        commodityName: Option<String>,
        #[serde(flatten)]
        extra: HashMap<String, Value>,
    }

    #[derive(Serialize, Deserialize)]
    struct ResMsg {
        message: String,
        method: Option<String>,
        code: String,
    }

    #[derive(Serialize, Deserialize)]
    #[allow(non_snake_case)]
    struct Response {
        datas: Vec<SwapMarket>,
        resMsg: ResMsg,
    }

    let txt = http_get("https://www.zbg.com/exchange/api/v1/future/common/contracts").unwrap();
    let resp = serde_json::from_str::<Response>(&txt).unwrap();
    let swap_markets = resp.datas;

    let mut mapping = HashMap::<i64, SwapContractInfo>::new();
    for swap_market in swap_markets.iter() {
        let contract_info = SwapContractInfo {
            symbol: swap_market.symbol.clone(),
            contract_id: swap_market.contractId,
            contract_unit: swap_market.contractUnit.parse::<f64>().unwrap(),
        };
        mapping.insert(contract_info.contract_id, contract_info);
    }
    mapping
}

// https://mxcdevelop.github.io/APIDoc/contract.api.cn.html#4483df6e28
#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct RawTradeMsg {
    p: f64, // price
    v: f64, // quantity
    T: i64, // 1, buy; 2, sell
    t: i64, // timestamp
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
struct WebsocketMsg<T: Sized> {
    channel: String,
    symbol: String,
    ts: i64,
    data: T,
}

fn calc_quantity_and_volume(
    market_type: MarketType,
    contract_id: i64,
    price: f64,
    size: f64,
) -> (f64, f64) {
    match market_type {
        MarketType::InverseSwap => {
            let contract_unit = SWAP_CONTRACT_MAP.get(&contract_id).unwrap().contract_unit;
            let volume = size * contract_unit;

            (volume / price, volume)
        }
        MarketType::LinearSwap => {
            let contract_unit = SWAP_CONTRACT_MAP.get(&contract_id).unwrap().contract_unit;
            let quantity = size * contract_unit;

            (quantity, quantity * price)
        }
        _ => panic!("Unknown market_type {}", market_type),
    }
}

pub(super) fn parse_trade(market_type: MarketType, msg: &str) -> Result<Vec<TradeMsg>> {
    #[derive(Serialize, Deserialize)]
    #[allow(non_snake_case)]
    struct RawTradeMsg {
        contractId: i64,
        trades: Vec<Value>,
    }

    let ws_msg = serde_json::from_str::<Vec<Value>>(msg)?;
    assert_eq!(ws_msg[0].as_str().unwrap(), "future_tick");
    let raw_trade: RawTradeMsg = serde_json::from_value(ws_msg[1].clone()).unwrap();

    let contract_info = SWAP_CONTRACT_MAP.get(&raw_trade.contractId).unwrap();
    let symbol = contract_info.symbol.as_str();
    let pair = crypto_pair::normalize_pair(symbol, EXCHANGE_NAME).unwrap();

    let timestamp = raw_trade.trades[0].as_i64().unwrap();
    let price = raw_trade.trades[1]
        .as_str()
        .unwrap()
        .parse::<f64>()
        .unwrap();
    let size = raw_trade.trades[2]
        .as_str()
        .unwrap()
        .parse::<f64>()
        .unwrap();
    let side = if raw_trade.trades[3].as_i64().unwrap() == -1 {
        TradeSide::Sell
    } else {
        TradeSide::Buy
    };

    let (quantity, volume) =
        calc_quantity_and_volume(market_type, contract_info.contract_id, price, size);

    let trade = TradeMsg {
        exchange: EXCHANGE_NAME.to_string(),
        market_type,
        symbol: symbol.to_string(),
        pair,
        msg_type: MessageType::Trade,
        timestamp: timestamp / 1000,
        price,
        quantity,
        volume,
        side,
        trade_id: timestamp.to_string(),
        raw: serde_json::to_value(&raw_trade).unwrap(),
    };

    Ok(vec![trade])
}
