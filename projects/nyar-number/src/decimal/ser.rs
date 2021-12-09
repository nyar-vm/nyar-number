use crate::NyarDecimal;
use serde::{ser::SerializeStruct, Serialize, Serializer};

impl Serialize for NyarDecimal {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut ser = Serializer::serialize_struct(serializer, "Decimal", 3)?;
        ser.serialize_field("type", "decimal")?;
        ser.serialize_field("sign", &self.sign)?;
        ser.serialize_field("value", &*self.digits.get())?;
        ser.end()
    }
}
