use crate::NyarDigits;
use serde::{Serialize, Serializer};

impl Serialize for NyarDigits {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self._repr.get().serialize(serializer)
    }
}
