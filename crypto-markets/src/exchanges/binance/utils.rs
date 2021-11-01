use super::super::utils::http_get;
use crate::error::{Error, Result};

use serde_json::Value;
use std::collections::HashMap;

fn check_code_in_body(resp: String) -> Result<String> {
    let obj = serde_json::from_str::<HashMap<String, Value>>(&resp);
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

pub(super) fn binance_http_get(url: &str) -> Result<String> {
    let ret = http_get(url, None);
    match ret {
        Ok(resp) => check_code_in_body(resp),
        Err(_) => ret,
    }
}

pub(super) fn parse_filter<'a>(
    filters: &'a [HashMap<String, Value>],
    filter_type: &'a str,
    field: &'static str,
) -> &'a str {
    filters
        .iter()
        .find(|x| x["filterType"] == filter_type)
        .unwrap()[field]
        .as_str()
        .unwrap()
}
