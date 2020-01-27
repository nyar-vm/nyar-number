use crate::{NyarDecimal, NyarReal};
use serde::{de::Error, Deserialize, Deserializer};

impl<'de> Deserialize<'de> for NyarReal {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        if let Ok(o) = NyarDecimal::deserialize(deserializer) {
            return Ok(NyarReal::Decimal(o));
        }
        Err(Error::missing_field("type"))
    }
}
