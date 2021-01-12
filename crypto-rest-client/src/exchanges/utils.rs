use crate::error::Result;
use std::collections::HashMap;

pub(super) fn http_get(url: &str, params: &HashMap<String, String>) -> Result<String> {
    let mut full_url = url.to_string();
    let mut first = true;
    for (k, v) in params.iter() {
        if first {
            full_url.push_str(format!("?{}={}", k, v).as_str());
            first = false;
        } else {
            full_url.push_str(format!("&{}={}", k, v).as_str());
        }
    }
    // println!("{}", full_url);

    let client = reqwest::blocking::Client::builder()
         .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/87.0.4280.88 Safari/537.36")
         .build()?;
    let response = client.get(full_url.as_str()).send()?.text()?;

    Ok(response)
}

macro_rules! gen_api {
    ( $path:expr$(, $param_name:ident )* ) => {
        {
            #[allow(unused_mut)]
            let mut params = HashMap::new();
            $(
                if let Some(param_name) = $param_name {
                    params.insert(stringify!($param_name).to_string(), param_name.to_string());
                }
            )*
            let url = if $path.starts_with("http") { $path.to_string() } else { format!("{}{}",BASE_URL, $path) };
            http_get(&url, &params)
        }
    }
}
