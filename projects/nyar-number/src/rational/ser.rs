use crate::NyarRational;
use serde::{ser::SerializeStruct, Serialize, Serializer};

impl Serialize for NyarRational {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut ser = Serializer::serialize_struct(serializer, "Rational", 3)?;
        ser.serialize_field("type", "rational")?;
        ser.serialize_field("sign", &self.sign)?;
        ser.serialize_field("numerator", &self.numerator.get().delegate())?;
        ser.serialize_field("denominator", &self.denominator.get().delegate())?;
        ser.end()
    }
}
