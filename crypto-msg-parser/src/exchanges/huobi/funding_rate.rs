use crypto_market_type::MarketType;
use crypto_msg_type::MessageType;

use crate::FundingRateMsg;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use simple_error::SimpleError;
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
pub(super) struct WebsocketMsg {
    op: String,
    pub topic: String,
    ts: i64,
    data: Vec<RawFundingRateMsg>,
}

pub(super) fn parse_funding_rate(
    market_type: MarketType,
    msg: &str,
) -> Result<Vec<FundingRateMsg>, SimpleError> {
    let ws_msg = serde_json::from_str::<WebsocketMsg>(msg)
        .map_err(|_e| SimpleError::new(format!("Failed to deserialize {} to WebsocketMsg", msg)))?;
    let mut funding_rates: Vec<FundingRateMsg> = ws_msg
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
            json: serde_json::to_string(&raw_msg).unwrap(),
        })
        .collect();
    if funding_rates.len() == 1 {
        funding_rates[0].json = msg.to_string();
    }
    Ok(funding_rates)
}
