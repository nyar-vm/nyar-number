use crate::NyarInteger;
use serde::{ser::SerializeStruct, Serialize, Serializer};

impl Serialize for NyarInteger {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut ser = Serializer::serialize_struct(serializer, "Integer", 3)?;
        ser.serialize_field("type", "integer")?;
        ser.serialize_field("sign", &self.sign)?;
        ser.serialize_field("value", &self.digits)?;
        ser.end()
    }
}
