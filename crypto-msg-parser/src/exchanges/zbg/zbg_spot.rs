use std::collections::{BTreeMap, HashMap};

use super::super::utils::{convert_timestamp, http_get};
use crypto_market_type::MarketType;
use crypto_msg_type::MessageType;

use crypto_message::{Order, OrderBookMsg, TradeMsg, TradeSide};

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use simple_error::SimpleError;

use super::EXCHANGE_NAME;

static SYMBOL_MAP: Lazy<HashMap<i64, String>> = Lazy::new(|| {
    // offline data, in case the network is down
    let mut m: HashMap<i64, String> = vec![
        (320, "eth_qc"),
        (321, "zb_usdt"),
        (329, "btc_usdt"),
        (330, "eth_usdt"),
        (331, "ltc_usdt"),
        (333, "eos_usdt"),
        (336, "zt_usdt"),
        (354, "btmc_zt"),
        (356, "abc_usdt"),
        (364, "zt_qc"),
        (374, "usdt_qc"),
        (380, "ada_usdt"),
        (382, "etc_usdt"),
        (386, "xrp_usdt"),
        (393, "comc_qc"),
        (411, "abbc_usdt"),
        (415, "bch_usdt"),
        (416, "bsv_usdt"),
        (417, "dash_usdt"),
        (466, "doge_usdt"),
        (476, "xlm_usdt"),
        (477, "trx_usdt"),
        (5008, "qtum_qc"),
        (5009, "qtum_usdt"),
        (5063, "ggt_usdt"),
        (5073, "520_qc"),
        (5109, "bnct_usdt"),
        (5150, "bts_usdt"),
        (5173, "torocus20_btc"),
        (5186, "scc_qc"),
        (5196, "xmr_usdt"),
        (5205, "kok_usdt"),
        (5209, "btc_qc"),
        (5211, "vlx_usdt"),
        (5212, "vlx_btc"),
        (5213, "vlx_eth"),
        (5214, "zb_qc"),
        (5215, "xwc_qc"),
        (5216, "xwc_usdt"),
        (5223, "ksm_usdt"),
        (5229, "arec_usdt"),
        (5235, "dot_qc"),
        (5238, "gdfc_qc"),
        (5241, "dot_usdt"),
        (5248, "uni_usdt"),
        (5249, "nbs_usdt"),
        (5256, "fil_usdt"),
        (5258, "woo_usdt"),
        (5259, "near_usdt"),
        (5260, "hep_usdt"),
        (5261, "xt_usdt"),
        (5262, "sxc_usdt"),
        (5263, "glc_usdt"),
        (5264, "wmc_usdt"),
        (5266, "fd_usdt"),
        (5267, "lon_usdt"),
        (5268, "cru_usdt"),
        (5270, "a5t_usdt"),
        (5271, "etha_usdt"),
        (5272, "idv_usdt"),
        (5273, "rc_usdt"),
        (5274, "lpt_usdt"),
        (5275, "dora_usdt"),
        (5276, "math_usdt"),
        (5277, "ht_usdt"),
        (5278, "mdx_usdt"),
        (5279, "bgg_usdt"),
        (5280, "rmsc_usdt"),
        (5281, "dfl_usdt"),
        (5282, "efil_usdt"),
        (5283, "safemoon_usdt"),
        (5285, "lemd_usdt"),
        (5286, "xch_usdt"),
        (5287, "xmpt_usdt"),
        (5288, "cspr_usdt"),
        (5289, "sol_usdt"),
        (5290, "dog_usdt"),
        (5294, "fly_usdt"),
        (5295, "bzz_usdt"),
        (5297, "nac_usdt"),
        (5298, "svt_usdt"),
        (5299, "ltc_qc"),
        (5300, "doge_qc"),
        (5301, "dash_qc"),
        (5302, "xrp_qc"),
        (5303, "trx_qc"),
        (5305, "xlm_qc"),
        (5306, "xem_qc"),
        (5307, "btm_qc"),
        (5308, "true_qc"),
        (5310, "kerri_usdt"),
        (5311, "enj_usdt"),
        (5312, "matic_usdt"),
        (5313, "chz_usdt"),
        (5314, "aave_usdt"),
        (5315, "neo_usdt"),
        (5316, "atom_usdt"),
        (5317, "comp_usdt"),
        (5318, "algo_usdt"),
        (5319, "yfi_usdt"),
        (5320, "sand_usdt"),
        (5321, "mana_qc"),
        (5322, "sana_qc"),
        (5323, "nfg_usdt"),
        (5324, "aot_usdt"),
        (5325, "dydx_usdt"),
        (5326, "lbt2_usdt"),
        (5327, "glc_qc"),
        (5328, "zbtc3s_zusd"),
        (5329, "zbtc3l_zusd"),
        (5330, "ens_usdt"),
        (5331, "btc3l_usdt"),
        (5332, "nabox_usdt"),
        (5333, "btv_usdt"),
        (5334, "btc3s_usdt"),
        (5335, "kilt_usdt"),
        (5336, "people_usdt"),
        (5337, "eth3s_usdt"),
        (5338, "eth3l_usdt"),
        (5339, "sgb_usdt"),
        (5340, "aurora_usdt"),
        (5341, "efi_usdt"),
        (5342, "shib_usdt"),
        (5343, "sdn_usdt"),
        (5344, "bnb3s_usdt"),
        (5345, "bnb3l_usdt"),
        (5346, "sol3s_usdt"),
        (5347, "sol3l_usdt"),
        (5348, "ada3l_usdt"),
        (5349, "ada3s_usdt"),
        (5350, "xrp3l_usdt"),
        (5351, "xrp3s_usdt"),
        (5352, "luna3l_usdt"),
        (5353, "luna3s_usdt"),
        (5354, "imx_usdt"),
        (5355, "nu_usdt"),
        (5356, "1inch_usdt"),
        (5357, "gtc_usdt"),
        (5358, "cvx_usdt"),
        (5359, "xym_qc"),
        (5360, "cart_usdt"),
        (5361, "ilv_usdt"),
        (5362, "tvk_usdt"),
        (5363, "alcx_usdt"),
        (5364, "paf_usdt"),
        (5365, "powr_usdt"),
        (5366, "thg_usdt"),
        (5367, "bnx_usdt"),
        (5368, "dar_usdt"),
        (5369, "vgx_usdt"),
        (5370, "suku_usdt"),
        (5371, "looks_usdt"),
        (5372, "wtf_usdt"),
        (5373, "justice_usdt"),
        (5374, "sos_usdt"),
        (5375, "nct_usdt"),
        (5376, "rss3_usdt"),
        (5377, "lqty_usdt"),
        (5378, "inv_usdt"),
        (5379, "gno_usdt"),
        (5380, "cere_usdt"),
        (5381, "ape_usdt"),
        (5382, "aca_usdt"),
        (5383, "glmr_usdt"),
        (5384, "avax_usdt"),
        (5385, "crf_usdt"),
        (5386, "dia_usdt"),
        (5387, "rlc_usdt"),
        (5388, "gmt_usdt"),
        (5389, "usdc_qc"),
        (5390, "usdc_usdt"),
        (5391, "strm_usdt"),
        (5392, "entc_usdt"),
        (5393, "sch_usdt"),
        (5394, "gotg_usdt"),
    ]
    .into_iter()
    .map(|x| (x.0, x.1.to_string()))
    .collect();

    let from_online = fetch_symbol_info();
    for (pair, contract_value) in from_online {
        m.insert(pair, contract_value);
    }

    m
});

// See https://zbgapi.github.io/docs/spot/v1/en/#public-get-all-supported-trading-symbols
fn fetch_symbol_info() -> BTreeMap<i64, String> {
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "kebab-case")]
    struct SpotMarket {
        symbol: String,
        id: String,
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
        datas: Vec<SpotMarket>,
        resMsg: ResMsg,
    }

    let mut mapping = BTreeMap::<i64, String>::new();
    if let Ok(txt) = http_get("https://www.zbg.com/exchange/api/v1/common/symbols") {
        let resp = serde_json::from_str::<Response>(&txt).unwrap();
        let spot_markets = resp.datas;

        for spot_market in spot_markets.iter() {
            mapping.insert(
                spot_market.id.parse::<i64>().unwrap(),
                spot_market.symbol.clone(),
            );
        }
    }

    mapping
}

// NOTE:zbg spot websocket sometimes returns lowercase symbols, and sometimes
// returns uppercase, which is very annoying, thus we unify to lowercase here
pub(super) fn extract_symbol(msg: &str) -> Result<String, SimpleError> {
    if msg.contains("datas") && msg.contains("resMsg") {
        // RESTful
        let obj = serde_json::from_str::<HashMap<String, Value>>(msg).unwrap();
        return if let Some(symbol) = obj.get("symbol") {
            Ok(symbol.as_str().unwrap().to_string())
        } else {
            Ok("NONE".to_string())
        };
    }

    // websocket
    if msg.starts_with(r#"{"trade_statistic":[["#) {
        let obj = serde_json::from_str::<HashMap<String, Value>>(msg).map_err(SimpleError::from)?;
        let trade_statistic = obj["trade_statistic"].as_array().unwrap();
        let ret = if trade_statistic.len() > 1 {
            Ok("ALL".to_string())
        } else {
            let symbol_id = trade_statistic[0][0]
                .as_str()
                .unwrap()
                .parse::<i64>()
                .unwrap();
            Ok(SYMBOL_MAP.get(&symbol_id).expect(msg).clone())
        };
        return ret;
    }
    let arr = if let Ok(list) = serde_json::from_str::<Vec<Vec<Value>>>(msg) {
        list[0].clone()
    } else if let Ok(arr) = serde_json::from_str::<Vec<Value>>(msg) {
        arr
    } else {
        return Err(SimpleError::new(format!(
            "Failed to extract symbol from {}",
            msg
        )));
    };
    let msg_type = arr[0].as_str().unwrap();
    match msg_type {
        "T" | "E" => Ok(arr[3].as_str().unwrap().to_lowercase()),
        "K" | "AE" => Ok(arr[2].as_str().unwrap().to_lowercase()),
        _ => Err(SimpleError::new(format!(
            "Unsupported msg_type {} in {}",
            msg_type, msg
        ))),
    }
}

pub(super) fn extract_timestamp(msg: &str) -> Result<Option<i64>, SimpleError> {
    if msg.contains("datas") && msg.contains("resMsg") {
        // RESTful
        let obj = serde_json::from_str::<HashMap<String, Value>>(msg).unwrap();
        return if let Some(timestamp) = obj["datas"].get("timestamp") {
            Ok(convert_timestamp(timestamp))
        } else {
            Ok(None)
        };
    }

    // websocket
    if msg.starts_with(r#"{"trade_statistic":[["#) {
        return Ok(None); // trade_statistic doesn't have timestamp
    }
    let arr_2d = if let Ok(list) = serde_json::from_str::<Vec<Vec<Value>>>(msg) {
        list
    } else if let Ok(list) = serde_json::from_str::<Vec<Value>>(msg) {
        vec![list]
    } else {
        return Err(SimpleError::new(format!(
            "Failed to extract symbol from {}",
            msg
        )));
    };

    let timestamp = arr_2d
        .iter()
        .filter(|arr| {
            let msg_type = arr[0].as_str().unwrap();
            match msg_type {
                "T" | "E" => arr[2].is_string() && arr[2].as_str().unwrap().parse::<i64>().is_ok(),
                "K" | "AE" => arr[3].is_string() && arr[3].as_str().unwrap().parse::<i64>().is_ok(),
                _ => false,
            }
        })
        .map(|arr| {
            let msg_type = arr[0].as_str().unwrap();
            match msg_type {
                "T" | "E" => convert_timestamp(&arr[2]).unwrap(),
                "K" | "AE" => convert_timestamp(&arr[3]).unwrap(),
                _ => panic!("Not possible {}", msg),
            }
        })
        .max();
    Ok(timestamp)
}

// r#"[["AE","5319","YFI_USDT",null,{"asks":null},{"bids":null}]]"#;

// https://zbgapi.github.io/docs/spot/v1/en/#market-trade
// [T, symbol-id, symbol, timestamp, ask/bid, price, quantity]
pub(super) fn parse_trade(msg: &str) -> Result<Vec<TradeMsg>, SimpleError> {
    let arr = if msg.starts_with(r#"[["T","#) {
        serde_json::from_str::<Vec<Vec<String>>>(msg).map_err(|_e| {
            SimpleError::new(format!("Failed to deserialize {} to Vec<Vec<String>>", msg))
        })?
    } else if msg.starts_with(r#"["T","#) {
        let tmp = serde_json::from_str::<Vec<String>>(msg).map_err(|_e| {
            SimpleError::new(format!("Failed to deserialize {} to Vec<String>", msg))
        })?;
        vec![tmp]
    } else {
        return Err(SimpleError::new(format!("Invalid trade msg {}", msg)));
    };

    let mut trades: Vec<TradeMsg> = arr
        .into_iter()
        .map(|raw_trade| {
            assert_eq!(raw_trade[0], "T");
            let timestamp = raw_trade[2].parse::<i64>().unwrap() * 1000;
            let symbol = raw_trade[3].as_str();
            let side = if raw_trade[4] == "ask" {
                TradeSide::Sell
            } else {
                TradeSide::Buy
            };
            let price = raw_trade[5].parse::<f64>().unwrap();
            let quantity = raw_trade[6].parse::<f64>().unwrap();

            TradeMsg {
                exchange: EXCHANGE_NAME.to_string(),
                market_type: MarketType::Spot,
                symbol: symbol.to_lowercase(),
                pair: crypto_pair::normalize_pair(symbol, EXCHANGE_NAME).unwrap(),
                msg_type: MessageType::Trade,
                timestamp,
                price,
                quantity_base: quantity,
                quantity_quote: price * quantity,
                quantity_contract: None,
                side,
                trade_id: timestamp.to_string(),
                json: serde_json::to_string(&raw_trade).unwrap(),
            }
        })
        .collect();

    if trades.len() == 1 {
        trades[0].json = msg.to_string();
    }
    Ok(trades)
}

#[derive(Serialize, Deserialize)]
struct OrderbookSnapshot {
    asks: Vec<[String; 2]>,
    bids: Vec<[String; 2]>,
}

// https://zbgapi.github.io/docs/spot/v1/en/#market-depth
// snapshot：
// [AE, symbol-id, symbol, timestamp, asks:[[price, quantity]], bids[[price, quantity]]]
// update:
// [E, symbol-id, timestamp, symbol, ask/bid, price, quantity]
pub(crate) fn parse_l2(msg: &str) -> Result<Vec<OrderBookMsg>, SimpleError> {
    let snapshot = msg.starts_with(r#"[["AE","#);

    let orderbooks = if snapshot {
        let arr = serde_json::from_str::<Vec<Vec<Value>>>(msg).map_err(|_e| {
            SimpleError::new(format!("Failed to deserialize {} to Vec<Vec<Value>>", msg))
        })?;

        let parse_order = |raw_order: &[Value; 2]| -> Order {
            let price: f64 = if raw_order[0].is_string() {
                raw_order[0].as_str().unwrap().parse::<f64>().unwrap()
            } else if raw_order[0].is_f64() || raw_order[0].is_i64() || raw_order[0].is_u64() {
                raw_order[0].as_f64().unwrap()
            } else {
                panic!("Unknown format {}", msg);
            };

            let quantity_base: f64 = if raw_order[1].is_string() {
                raw_order[1].as_str().unwrap().parse::<f64>().unwrap()
            } else if raw_order[1].is_f64() || raw_order[1].is_i64() || raw_order[1].is_u64() {
                raw_order[1].as_f64().unwrap()
            } else {
                panic!("Unknown format {}", msg);
            };

            Order {
                price,
                quantity_base,
                quantity_quote: price * quantity_base,
                quantity_contract: None,
            }
        };

        let mut v = arr
            .iter()
            .filter(|raw_orderbook| !raw_orderbook[3].is_null())
            .map(|raw_orderbook| {
                let symbol = raw_orderbook[2].as_str().unwrap();
                let pair = crypto_pair::normalize_pair(symbol, EXCHANGE_NAME).unwrap();
                let timestamp = raw_orderbook[3].as_str().unwrap().parse::<i64>().unwrap() * 1000;

                let asks = serde_json::from_value::<Vec<[Value; 2]>>(
                    raw_orderbook[4]
                        .as_object()
                        .unwrap()
                        .get("asks")
                        .unwrap()
                        .clone(),
                )
                .unwrap()
                .iter()
                .map(|x| parse_order(x))
                .collect::<Vec<Order>>();
                let bids = serde_json::from_value::<Vec<[Value; 2]>>(
                    raw_orderbook[5]
                        .as_object()
                        .unwrap()
                        .get("bids")
                        .unwrap()
                        .clone(),
                )
                .unwrap()
                .iter()
                .map(|x| parse_order(x))
                .collect::<Vec<Order>>();

                OrderBookMsg {
                    exchange: EXCHANGE_NAME.to_string(),
                    market_type: MarketType::Spot,
                    symbol: symbol.to_lowercase(),
                    pair,
                    msg_type: MessageType::L2Event,
                    timestamp,
                    seq_id: None,
                    prev_seq_id: None,
                    asks,
                    bids,
                    snapshot,
                    json: serde_json::to_string(raw_orderbook)
                        .unwrap()
                        .as_str()
                        .to_string(),
                }
            })
            .collect::<Vec<OrderBookMsg>>();

        if v.len() == 1 {
            v[0].json = msg.to_string();
        }
        v
    } else {
        let arr = serde_json::from_str::<Vec<String>>(msg).map_err(|_e| {
            SimpleError::new(format!("Failed to deserialize {} to Vec<String>", msg))
        })?;
        let symbol = arr[3].to_lowercase();
        let pair = crypto_pair::normalize_pair(&symbol, EXCHANGE_NAME).ok_or_else(|| {
            SimpleError::new(format!("Failed to normalize {} from {}", symbol, msg))
        })?;
        let timestamp = arr[2].parse::<i64>().unwrap() * 1000;

        let mut asks: Vec<Order> = Vec::new();
        let mut bids: Vec<Order> = Vec::new();

        let order: Order = {
            let price = arr[5].parse::<f64>().unwrap();
            let quantity_base = arr[6].parse::<f64>().unwrap();

            Order {
                price,
                quantity_base,
                quantity_quote: quantity_base * price,
                quantity_contract: None,
            }
        };

        if arr[4] == "BID" {
            bids.push(order);
        } else {
            asks.push(order);
        }

        let orderbook = OrderBookMsg {
            exchange: EXCHANGE_NAME.to_string(),
            market_type: MarketType::Spot,
            symbol,
            pair,
            msg_type: MessageType::L2Event,
            timestamp,
            seq_id: None,
            prev_seq_id: None,
            asks,
            bids,
            snapshot,
            json: msg.to_string(),
        };
        vec![orderbook]
    };
    Ok(orderbooks)
}

#[cfg(test)]
mod tests {
    use super::fetch_symbol_info;

    #[test]
    #[ignore]
    fn print_contract_values() {
        let mut mapping = fetch_symbol_info();
        // merge
        for (symbol_id, info) in super::SYMBOL_MAP.iter() {
            if !mapping.contains_key(symbol_id) {
                mapping.insert(*symbol_id, info.clone());
            }
        }
        for (symbol_id, symbol) in mapping {
            println!("({}, \"{}\"),", symbol_id, symbol,);
        }
    }
}
