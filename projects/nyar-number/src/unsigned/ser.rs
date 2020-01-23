use crate::NyarUnsigned;
use num::BigUint;
use serde::{ser::SerializeStruct, Deserialize, Serialize, Serializer};

#[derive(Serialize, Deserialize)]
struct UnsignedWrap<'raw> {
    r#type: &'static str,
    value: &'raw BigUint,
}

impl Serialize for NyarUnsigned {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut ser = Serializer::serialize_struct(serializer, "Unsigned", 2)?;
        ser.serialize_field("type", "unsigned")?;
        ser.serialize_field("value", &self._repr)?;
        ser.end()
    }
}
