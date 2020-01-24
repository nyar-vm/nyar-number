use crate::{utils::RealVisitor, NyarInteger, NyarUnsigned};
use serde::{
    de::{Error, Unexpected},
    Deserialize, Deserializer,
};
use shredder::Gc;

impl<'de> Deserialize<'de> for NyarInteger {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let real = deserializer.deserialize_any(RealVisitor::default())?;
        if real.r#type != "integer" {
            // to warning
            Err(Error::invalid_type(Unexpected::Other(&real.r#type), &"type=integer"))?
        }
        Ok(Self { sign: real.sign, digits: Gc::new(NyarUnsigned { _repr: real.value.unwrap_or_default() }) })
    }
}