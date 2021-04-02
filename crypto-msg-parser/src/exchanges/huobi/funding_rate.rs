use crypto_market_type::MarketType;

use crate::{FundingRateMsg, MessageType};

use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct RawFundingRateMsg {
    symbol: String,
    contract_code: String,
    fee_asset: String,
    funding_time: String,
    funding_rate: String,
    estimated_rate: String,
    settlement_time: String,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
struct WebsocketMsg {
    op: String,
    topic: String,
    ts: i64,
    data: Vec<RawFundingRateMsg>,
}

pub(crate) fn parse_funding_rate(
    market_type: MarketType,
    msg: &str,
) -> Result<Vec<FundingRateMsg>> {
    let ws_msg = serde_json::from_str::<WebsocketMsg>(msg)?;
    let funding_rates = ws_msg
        .data
        .into_iter()
        .map(|raw_msg| FundingRateMsg {
            exchange: "huobi".to_string(),
            market_type,
            symbol: raw_msg.contract_code.clone(),
            pair: crypto_pair::normalize_pair(&raw_msg.contract_code, "huobi").unwrap(),
            msg_type: MessageType::FundingRate,
            timestamp: raw_msg.funding_time.parse::<i64>().unwrap(),
            funding_rate: raw_msg.funding_rate.parse::<f64>().unwrap(),
            funding_time: raw_msg.settlement_time.parse::<i64>().unwrap(),
            estimated_rate: Some(raw_msg.estimated_rate.parse::<f64>().unwrap()),
            raw: serde_json::from_str(msg).unwrap(),
        })
        .collect();
    Ok(funding_rates)
}
