use crate::NyarDigits;
use serde::{Deserialize, Deserializer};
use shredder::Gc;

impl<'de> Deserialize<'de> for NyarDigits {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let bytes = Vec::deserialize(deserializer)?;
        Ok(Self { _repr: Gc::new(bytes) })
    }
}
