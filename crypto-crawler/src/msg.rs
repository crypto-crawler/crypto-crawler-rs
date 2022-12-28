use crypto_market_type::MarketType;
use crypto_msg_type::MessageType;
use serde::{Deserialize, Serialize};
use std::{
    convert::TryInto,
    str::FromStr,
    time::{SystemTime, UNIX_EPOCH},
};

/// Message represents messages received by crawlers.
#[derive(Serialize, Deserialize)]
pub struct Message {
    /// The exchange name, unique for each exchage
    pub exchange: String,
    /// Market type
    pub market_type: MarketType,
    /// Message type
    pub msg_type: MessageType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    /// Unix timestamp in milliseconds
    pub received_at: u64,
    /// the original message
    pub json: String,
}

impl Message {
    pub fn new(
        exchange: String,
        market_type: MarketType,
        msg_type: MessageType,
        json: String,
    ) -> Self {
        Message {
            exchange,
            market_type,
            msg_type,
            symbol: None,
            received_at: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis()
                .try_into()
                .unwrap(),
            json: json.trim().to_string(),
        }
    }

    pub fn new_with_symbol(
        exchange: String,
        market_type: MarketType,
        msg_type: MessageType,
        symbol: String,
        json: String,
    ) -> Self {
        let mut msg = Self::new(exchange, market_type, msg_type, json);
        msg.symbol = Some(symbol);
        msg
    }

    /// Convert to a TSV string.
    ///
    /// The `exchange`, `market_type` and `msg_type` fields are not included to
    /// save some disk space.
    pub fn to_tsv_string(&self) -> String {
        let symbol = if let Some(symbol) = self.symbol.clone() { symbol } else { "".to_string() };
        format!("{}\t{}\t{}", self.received_at, symbol, self.json)
    }

    /// Convert from a TSV string.
    pub fn from_tsv_string(exchange: &str, market_type: &str, msg_type: &str, s: &str) -> Self {
        let v: Vec<&str> = s.split('\t').collect();
        assert_eq!(3, v.len());
        let market_type = MarketType::from_str(market_type).unwrap();
        let msg_type = MessageType::from_str(msg_type).unwrap();

        let symbol = if v[1].is_empty() { None } else { Some(v[1].to_string()) };

        Message {
            exchange: exchange.to_string(),
            market_type,
            msg_type,
            symbol,
            received_at: v[0].parse::<u64>().unwrap(),
            json: v[2].to_string(),
        }
    }
}

impl std::fmt::Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
