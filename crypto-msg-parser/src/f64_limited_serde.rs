use serde::{de::Deserializer, ser::Serializer, Deserialize};

/// Serialize a f64 to a string with 9 maximal decimals.
pub fn serialize<S>(num: &f64, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let s = format!("{:.9}", num);
    let x = s.as_str().trim_end_matches('0');
    serializer.serialize_str(x)
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    s.parse::<f64>().map_err(serde::de::Error::custom)
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    pub struct MyStruct {
        #[serde(with = "crate::f64_limited_serde")]
        pub price: f64,
    }

    #[test]
    fn test_serialize() {
        let my_struct = MyStruct {
            price: 1.2000000001,
        };
        let text = serde_json::to_string(&my_struct).unwrap();
        assert_eq!(text.as_str(), r#"{"price":"1.2"}"#);

        let my_struct = MyStruct {
            price: 1.2000000006,
        };
        let text = serde_json::to_string(&my_struct).unwrap();
        assert_eq!(text.as_str(), r#"{"price":"1.200000001"}"#);
    }

    #[test]
    fn order_deserialize() {
        let my_struct1 = serde_json::from_str::<MyStruct>(r#"{"price":"1.200000001"}"#).unwrap();
        assert_eq!(my_struct1.price, 1.200000001);
    }
}
