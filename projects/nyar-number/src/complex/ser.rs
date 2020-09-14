use crate::complex::NyarComplex;
use serde::{ser::SerializeStruct, Serialize, Serializer};

impl Serialize for NyarComplex {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut ser = Serializer::serialize_struct(serializer, "Complex", 3)?;
        ser.serialize_field("type", "complex")?;
        ser.serialize_field("re", &self.re)?;
        ser.serialize_field("im", &self.im)?;
        ser.end()
    }
}
