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

    let response = reqwest::blocking::get(full_url.as_str())?.text()?;
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
            http_get(format!("{}{}",BASE_URL, $path).as_str(), &params)
        }
    }
}
