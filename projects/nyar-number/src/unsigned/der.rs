use crate::NyarDigits;
use num::Zero;
use serde::{Deserialize, Deserializer};
use shredder::Gc;

impl<'de> Deserialize<'de> for NyarDigits {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let bytes = Vec::deserialize(deserializer)?;
        if bytes.is_empty() { Ok(Self::zero()) } else { Ok(Self { _repr: Gc::new(bytes) }) }
    }
}
