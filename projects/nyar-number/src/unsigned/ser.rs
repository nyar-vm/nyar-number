use crate::NyarUnsigned;
use serde::{ser::SerializeStruct, Serialize, Serializer};

impl Serialize for NyarUnsigned {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut ser = Serializer::serialize_struct(serializer, "Unsigned", 2)?;
        ser.serialize_field("type", "unsigned")?;
        ser.serialize_field("value", &*self._repr.get())?;
        ser.end()
    }
}
