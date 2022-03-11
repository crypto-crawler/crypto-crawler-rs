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
            let code_int = code.as_i64().unwrap();
            if code_int == 0 || code_int == 200 {
                Ok(resp)
            } else {
                Err(Error(resp))
            }
        }
        None => Ok(resp),
    }
}

pub(super) fn mexc_http_get(url: &str) -> Result<String> {
    let ret = http_get(url, None);
    match ret {
        Ok(resp) => check_code_in_body(resp),
        Err(_) => ret,
    }
}
