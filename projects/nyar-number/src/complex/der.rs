use crate::{complex::NyarComplex, utils::RealVisitor};
use serde::{
    de::{Error, Unexpected},
    Deserialize, Deserializer,
};

impl<'de> Deserialize<'de> for NyarComplex {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let real = deserializer.deserialize_any(RealVisitor::default())?;
        if real.r#type != "complex" {
            // to warning
            Err(Error::invalid_type(Unexpected::Other(&real.r#type), &"type=complex"))?
        }
        let re = real.re.unwrap_or_default();
        let im = real.im.unwrap_or_default();
        Ok(Self { re, im })
    }
}
