use crate::NyarNumber;
use serde::{Deserialize, Deserializer};

impl<'de> Deserialize<'de> for NyarNumber {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
      todo!()
    }
}