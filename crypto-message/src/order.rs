use serde::{
    de::{Deserializer, SeqAccess, Visitor},
    ser::{SerializeSeq, Serializer},
    Deserialize, Serialize,
};

/// An order in the orderbook asks or bids array.
pub struct Order {
    /// price
    pub price: f64,
    // Number of base coins, 0 means the price level can be removed.
    pub quantity_base: f64,
    // Number of quote coins(mostly USDT)
    pub quantity_quote: f64,
    /// Number of contracts, always None for Spot
    pub quantity_contract: Option<f64>,
}

impl PartialEq for Order {
    fn eq(&self, other: &Self) -> bool {
        self.price == other.price
            && self.quantity_base == other.quantity_base
            && self.quantity_quote == other.quantity_quote
            && self.quantity_contract == other.quantity_contract
    }
}

impl Eq for Order {}

impl Serialize for Order {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let len: usize = if self.quantity_contract.is_some() {
            4
        } else {
            3
        };
        let mut seq = serializer.serialize_seq(Some(len))?;
        seq.serialize_element(&self.price)?;
        // limit the number of decimals to 9
        let quantity_base = format!("{:.9}", self.quantity_base)
            .as_str()
            .parse::<f64>()
            .unwrap();
        let quantity_quote = format!("{:.9}", self.quantity_quote)
            .as_str()
            .parse::<f64>()
            .unwrap();
        seq.serialize_element(&quantity_base)?;
        seq.serialize_element(&quantity_quote)?;
        if let Some(qc) = self.quantity_contract {
            seq.serialize_element(&qc)?;
        }

        seq.end()
    }
}

struct OrderVisitor;

impl<'de> Visitor<'de> for OrderVisitor {
    type Value = Order;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a nonempty sequence of numbers")
    }

    fn visit_seq<V>(self, mut visitor: V) -> Result<Order, V::Error>
    where
        V: SeqAccess<'de>,
    {
        let mut vec = Vec::<f64>::new();

        while let Some(elem) = visitor.next_element()? {
            vec.push(elem);
        }

        let order = Order {
            price: vec[0],
            quantity_base: vec[1],
            quantity_quote: vec[2],
            quantity_contract: if vec.len() == 4 { Some(vec[3]) } else { None },
        };

        Ok(order)
    }
}

impl<'de> Deserialize<'de> for Order {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(OrderVisitor)
    }
}

#[cfg(test)]
mod tests {
    use crate::order::Order;

    #[test]
    fn order_serialize() {
        let order = Order {
            price: 59999.8,
            quantity_base: 1.7000000001,
            quantity_quote: 59999.8 * 1.7,
            quantity_contract: Some(1.7),
        };
        let text = serde_json::to_string(&order).unwrap();
        assert_eq!(text.as_str(), "[59999.8,1.7,101999.66,1.7]");

        let order = Order {
            price: 59999.8,
            quantity_base: 1.7000000006,
            quantity_quote: 59999.8 * 1.7,
            quantity_contract: Some(1.7),
        };
        let text = serde_json::to_string(&order).unwrap();
        assert_eq!(text.as_str(), "[59999.8,1.700000001,101999.66,1.7]");
    }

    #[test]
    fn order_deserialize() {
        let expected = Order {
            price: 59999.8,
            quantity_base: 1.7,
            quantity_quote: 59999.8 * 1.7,
            quantity_contract: Some(1.7),
        };
        let actual = serde_json::from_str::<Order>("[59999.8,1.7,101999.66,1.7]").unwrap();
        assert_eq!(expected.price, actual.price);
        assert_eq!(expected.quantity_base, actual.quantity_base);
        assert_eq!(expected.quantity_quote, actual.quantity_quote);
        assert_eq!(expected.quantity_contract, actual.quantity_contract);
    }
}
