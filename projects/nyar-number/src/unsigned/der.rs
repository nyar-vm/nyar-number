use crate::{utils::RealVisitor, NyarDigits};
use serde::{
    de::{Error, Unexpected},
    Deserialize, Deserializer,
};

impl<'de> Deserialize<'de> for NyarDigits {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let real = deserializer.deserialize_any(RealVisitor::default())?;
        if real.r#type != "unsigned" {
            Err(Error::invalid_type(Unexpected::Other(&real.r#type), &"type=unsigned"))?
        }
        Ok(real.value.map(Self::from).unwrap_or_default())
    }
}
