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

    let client = reqwest::blocking::Client::builder()
         .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/87.0.4280.88 Safari/537.36")
         .build()?;
    let response = client.get(full_url.as_str()).send()?;

    if response.status().is_success() {
        Ok(response.text()?)
    } else {
        let ret = response.error_for_status();
        match ret {
            Ok(resp) => Err(Error(resp.text()?)),
            Err(e) => Err(Error::from(e)),
        }
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
}
