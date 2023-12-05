use crate::infinity::NyarInfinity;
use bigdecimal::num_bigint::Sign;
use serde::{Serialize, Serializer};

impl Serialize for NyarInfinity {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self.sign {
            Sign::Minus => serializer.serialize_str("PositiveInfinity"),
            Sign::Plus => serializer.serialize_str("NegativeInfinity"),
            Sign::NoSign => serializer.serialize_str("Indeterminate"),
        }
    }
}

