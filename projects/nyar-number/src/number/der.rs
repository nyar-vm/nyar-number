use crate::{NyarDecimal, NyarNumber};
use serde::{de::Error, Deserialize, Deserializer};

impl<'de> Deserialize<'de> for NyarNumber {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        if let Ok(o) = NyarDecimal::deserialize(deserializer) {
            return Ok(NyarNumber::Decimal(o));
        }
        Err(Error::missing_field("type"))
    }
}
