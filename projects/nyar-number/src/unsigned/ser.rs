use crate::NyarDigits;
use serde::{ser::SerializeSeq, Serialize, Serializer};

impl Serialize for NyarDigits {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let view = &**self._repr.get();
        let mut seq = serializer.serialize_seq(Some(view.len()))?;
        for element in view {
            seq.serialize_element(element)?;
        }
        seq.end()
    }
}
