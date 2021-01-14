use super::super::utils::http_get;
use crate::error::{Error, Result};

use serde_json::Value;
use std::collections::HashMap;

fn check_status_in_body(resp: String) -> Result<String> {
    let obj = serde_json::from_str::<HashMap<String, Value>>(&resp);
    if obj.is_err() {
        return Ok(resp);
    }

    match obj.unwrap().get("status") {
        Some(status) => {
            if status.as_str().unwrap() != "ok" {
                Err(Error(resp))
            } else {
                Ok(resp)
            }
        }
        None => Ok(resp),
    }
}

pub(super) fn huobi_http_get(url: &str) -> Result<String> {
    let ret = http_get(url, None);
    match ret {
        Ok(resp) => check_status_in_body(resp),
        Err(_) => ret,
    }
}
