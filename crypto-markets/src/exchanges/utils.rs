use reqwest::header;

use crate::error::{Error, Result};
use std::collections::HashMap;

pub(super) fn http_get(url: &str, params: Option<&HashMap<String, String>>) -> Result<String> {
    let mut full_url = url.to_string();
    if let Some(params) = params {
        let mut first = true;
        for (k, v) in params.iter() {
            if first {
                full_url.push_str(format!("?{}={}", k, v).as_str());
                first = false;
            } else {
                full_url.push_str(format!("&{}={}", k, v).as_str());
            }
        }
    }
    // println!("{}", full_url);

    let mut headers = header::HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, header::HeaderValue::from_static("application/json"));

    let client = reqwest::blocking::Client::builder()
         .default_headers(headers)
         .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/87.0.4280.88 Safari/537.36")
         .gzip(true)
         .build()?;
    let response = client.get(full_url.as_str()).send()?;

    match response.error_for_status() {
        Ok(resp) => Ok(resp.text()?),
        Err(error) => Err(Error::from(error)),
    }
}

#[allow(dead_code)]
fn precision_from_string(s: &str) -> i64 {
    if let Some(dot_pos) = s.find('.') {
        let mut none_zero = 0;
        for (i, ch) in s.chars().rev().enumerate() {
            if ch != '0' {
                none_zero = s.len() - 1 - i;
                break;
            }
        }
        (none_zero - dot_pos) as i64
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use serde_json::Value;

    // System proxies are enabled by default, see <https://docs.rs/reqwest/latest/reqwest/#proxies>
    #[test]
    #[ignore]
    fn use_system_socks_proxy() {
        std::env::set_var("https_proxy", "socks5://127.0.0.1:9050");
        let text = super::http_get("https://check.torproject.org/api/ip", None).unwrap();
        let obj = serde_json::from_str::<HashMap<String, Value>>(&text).unwrap();
        assert!(obj.get("IsTor").unwrap().as_bool().unwrap());
    }

    #[test]
    #[ignore]
    fn use_system_https_proxy() {
        std::env::set_var("https_proxy", "http://127.0.0.1:8118");
        let text = super::http_get("https://check.torproject.org/api/ip", None).unwrap();
        let obj = serde_json::from_str::<HashMap<String, Value>>(&text).unwrap();
        assert!(obj.get("IsTor").unwrap().as_bool().unwrap());
    }

    #[test]
    fn test_calc_precision() {
        assert_eq!(4, super::precision_from_string("0.000100"));
        assert_eq!(0, super::precision_from_string("10.00000000"));
    }
}
