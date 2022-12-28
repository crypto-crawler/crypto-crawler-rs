use std::collections::BTreeMap;

use crate::error::{Error, Result};

use once_cell::sync::Lazy;
use regex::Regex;
use serde_json::Value;

static SYMBOL_PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new("^[A-Z0-9-_.]{1,20}$").unwrap());

pub(super) fn check_symbol(symbol: &str) {
    if !SYMBOL_PATTERN.is_match(symbol) {
        panic!("Illegal symbol {}, legal symbol should be '^[A-Z0-9-_.]{{1,20}}$'.", symbol);
    }
}

pub(super) fn check_code_in_body(resp: String) -> Result<String> {
    let obj = serde_json::from_str::<BTreeMap<String, Value>>(&resp);
    if obj.is_err() {
        return Ok(resp);
    }

    match obj.unwrap().get("code") {
        Some(code) => {
            if code.as_i64().unwrap() != 0 {
                Err(Error(resp))
            } else {
                Ok(resp)
            }
        }
        None => Ok(resp),
    }
}

macro_rules! gen_api_binance {
    ( $path:expr$(, $param_name:ident )* ) => {
        {
            #[allow(unused_mut)]
            let mut params = BTreeMap::new();
            $(
                if let Some(param_name) = $param_name {
                    params.insert(stringify!($param_name).to_string(), param_name.to_string());
                }
            )*
            let ret = http_get(format!("{}{}",BASE_URL, $path).as_str(), &params);
            match ret {
                Ok(resp) => check_code_in_body(resp),
                Err(_) => ret,
            }
        }
    }
}
