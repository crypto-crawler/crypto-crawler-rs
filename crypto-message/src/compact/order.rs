use serde::{
    de::{Deserializer, SeqAccess, Visitor},
    ser::{SerializeSeq, Serializer},
    Deserialize, Serialize,
};
use strum_macros::{Display, EnumString};

/// Choose which field to use as the quantity.
#[derive(Copy, Clone, Debug, Display, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum QuantityChoice {
    /// quantity_base
    Base,
    /// quantity_quote
    Quote,
    /// quantity_contract
    Contract,
}

#[cfg(feature = "f32")]
pub type Float = f32;
#[cfg(not(feature = "f32"))]
pub type Float = f64;

/// An order in the orderbook asks or bids array.
#[derive(Copy, Clone)]
pub struct Order {
    /// price
    pub price: Float,
    // quantity, comes from one of quantity_base, quantity_quote and quantity_contract.
    pub quantity: Float,
}

impl PartialEq for Order {
    fn eq(&self, other: &Self) -> bool {
        self.price == other.price && self.quantity == other.quantity
    }
}

impl Eq for Order {}

impl Serialize for Order {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let len: usize = 2;
        let mut seq = serializer.serialize_seq(Some(len))?;
        seq.serialize_element(&self.price)?;
        // limit the number of decimals to 9
        let quantity = format!("{:.9}", self.quantity)
            .as_str()
            .parse::<f64>()
            .unwrap();
        seq.serialize_element(&quantity)?;

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
            price: vec[0] as Float,
            quantity: vec[1] as Float,
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
    use crate::compact::Order;

    #[test]
    fn order_serialize() {
        let order = Order {
            price: 59999.8,
            quantity: 1.7000000001,
        };
        let text = serde_json::to_string(&order).unwrap();
        assert_eq!(text.as_str(), "[59999.8,1.7]");

        let order = Order {
            price: 59999.8,
            quantity: 1.7000000006,
        };
        let text = serde_json::to_string(&order).unwrap();
        assert_eq!(text.as_str(), "[59999.8,1.700000001]");
    }

    #[test]
    fn order_deserialize() {
        let expected = Order {
            price: 59999.8,
            quantity: 1.7,
        };
        let actual = serde_json::from_str::<Order>("[59999.8,1.7,101999.66,1.7]").unwrap();
        assert_eq!(expected.price, actual.price);
        assert_eq!(expected.quantity, actual.quantity);
    }
}
