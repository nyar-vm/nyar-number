use crate::{utils::RealVisitor, NyarDecimal};
use serde::{
    de::{Error, Unexpected},
    Deserialize, Deserializer,
};

impl<'de> Deserialize<'de> for NyarDecimal {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let real = deserializer.deserialize_any(RealVisitor::default())?;
        if real.r#type != "decimal" {
            // to warning
            Err(Error::invalid_type(Unexpected::Other(&real.r#type), &"type=decimal"))?
        }
        Ok(Self { sign: real.sign, digits: Default::default(), scale: 0 })
    }
}
