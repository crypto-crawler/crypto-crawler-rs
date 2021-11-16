use crypto_market_type::MarketType;

use super::super::utils::http_get;
use crate::{MessageType, Order, OrderBookMsg, TradeMsg, TradeSide};

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use std::collections::{BTreeMap, HashMap};

const EXCHANGE_NAME: &str = "zbg";

lazy_static! {
    static ref SWAP_CONTRACT_MAP: HashMap<i64, SwapContractInfo> = {
        // offline data, in case the network is down
        let mut m: HashMap<i64, SwapContractInfo> = vec![
            (999999, "BTC_ZUSD", 0.01_f64),
            (1000000, "BTC_USDT", 0.01_f64),
            (1000001, "BTC_USD-R", 1_f64),
            (1000002, "ETH_USDT", 0.1_f64),
            (1000003, "ETH_USD-R", 1_f64),
            (1000008, "LTC_USDT", 0.1_f64),
            (1000009, "EOS_USDT", 1_f64),
            (1000010, "XRP_USDT", 10_f64),
            (1000011, "BCH_USDT", 0.1_f64),
            (1000012, "ETC_USDT", 1_f64),
            (1000013, "BSV_USDT", 0.1_f64),
            (1000014, "RHI_ZUSD", 0.01_f64),
            (1000015, "UNI_USDT", 0.1_f64),
            (1000016, "DOT_USDT", 1_f64),
            (1000017, "FIL_USDT", 0.1_f64),
            (1000018, "SUSHI_USDT", 1_f64),
            (1000019, "LINK_USDT", 1_f64),
            (1000020, "DOGE_USDT", 100_f64),
            (1000021, "AXS_USDT", 0.1_f64),
            (1000022, "ICP_USDT", 0.1_f64),
        ]
        .into_iter()
        .map(|x| (x.0, SwapContractInfo::new(x)))
        .collect();

        let from_online = fetch_swap_contracts();
        for (pair, contract_value) in from_online {
            m.insert(pair, contract_value);
        }

        m
    };
}

struct SwapContractInfo {
    contract_id: i64,
    symbol: String,
    contract_unit: f64,
}

impl SwapContractInfo {
    fn new(t: (i64, &str, f64)) -> Self {
        Self {
            contract_id: t.0,
            symbol: t.1.to_string(),
            contract_unit: t.2,
        }
    }
}

// See https://zbgapi.github.io/docs/future/v1/en/#public-get-contracts
fn fetch_swap_contracts() -> BTreeMap<i64, SwapContractInfo> {
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

    let mut mapping = BTreeMap::<i64, SwapContractInfo>::new();
    if let Ok(txt) = http_get("https://www.zbg.com/exchange/api/v1/future/common/contracts") {
        let resp = serde_json::from_str::<Response>(&txt).unwrap();
        let swap_markets = resp.datas;

        for swap_market in swap_markets.iter() {
            let contract_info = SwapContractInfo {
                symbol: swap_market.symbol.clone(),
                contract_id: swap_market.contractId,
                contract_unit: swap_market.contractUnit.parse::<f64>().unwrap(),
            };
            mapping.insert(contract_info.contract_id, contract_info);
        }
    }

    mapping
}

// https://www.zbgpro.com/docs/future/v1/cn/#7d44b7792d
#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct RawTradeMsg {
    contractId: i64,
    trades: Vec<Value>,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

// https://www.zbgpro.com/docs/future/v1/cn/#1529c9267f
#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct RawOrderbookMsg {
    contractId: i64,
    asks: Vec<[String; 2]>,
    bids: Vec<[String; 2]>,
    time: i64,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

pub(super) fn extract_symbol(_market_type: MarketType, msg: &str) -> Option<String> {
    let ws_msg = serde_json::from_str::<Vec<Value>>(msg).unwrap();
    let contract_id = ws_msg[1]["contractId"].as_i64().unwrap();
    let contract_info = SWAP_CONTRACT_MAP.get(&contract_id).unwrap();
    let symbol = contract_info.symbol.as_str();
    Some(symbol.to_string())
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

    let (quantity_base, quantity_quote) =
        calc_quantity_and_volume(market_type, contract_info.contract_id, price, size);

    let trade = TradeMsg {
        exchange: EXCHANGE_NAME.to_string(),
        market_type,
        symbol: symbol.to_string(),
        pair,
        msg_type: MessageType::Trade,
        timestamp: timestamp / 1000,
        price,
        quantity_base,
        quantity_quote,
        quantity_contract: Some(size),
        side,
        trade_id: timestamp.to_string(),
        json: msg.to_string(),
    };

    Ok(vec![trade])
}

pub(crate) fn parse_l2(market_type: MarketType, msg: &str) -> Result<Vec<OrderBookMsg>> {
    let ws_msg = serde_json::from_str::<Vec<Value>>(msg)?;
    assert_eq!(ws_msg[0].as_str().unwrap(), "future_snapshot_depth");
    let raw_orderbook: RawOrderbookMsg = serde_json::from_value(ws_msg[1].clone()).unwrap();

    let contract_info = SWAP_CONTRACT_MAP.get(&raw_orderbook.contractId).unwrap();
    let symbol = contract_info.symbol.as_str();
    let pair = crypto_pair::normalize_pair(symbol, EXCHANGE_NAME).unwrap();

    let parse_order = |raw_order: &[String; 2]| -> Order {
        let price = raw_order[0].parse::<f64>().unwrap();
        let quantity = raw_order[1].parse::<f64>().unwrap();
        let (quantity_base, quantity_quote) =
            calc_quantity_and_volume(market_type, contract_info.contract_id, price, quantity);

        Order {
            price,
            quantity_base,
            quantity_quote,
            quantity_contract: Some(quantity),
        }
    };

    let orderbook = OrderBookMsg {
        exchange: EXCHANGE_NAME.to_string(),
        market_type,
        symbol: symbol.to_string(),
        pair,
        msg_type: MessageType::L2Event,
        timestamp: raw_orderbook.time / 1000,
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
        snapshot: false,
        json: msg.to_string(),
    };

    Ok(vec![orderbook])
}

#[cfg(test)]
mod tests {
    use super::fetch_swap_contracts;

    #[test]
    fn print_contract_values() {
        let mapping = fetch_swap_contracts();
        for (_, contract) in mapping {
            println!(
                "({}, \"{}\", {}_f64),",
                contract.contract_id, contract.symbol, contract.contract_unit
            );
        }
    }
}
