use crate::NyarReal;
use serde::{Serialize, Serializer};

impl Serialize for NyarReal {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            NyarReal::Rational(v) => v.serialize(serializer),
            NyarReal::Decimal(v) => v.serialize(serializer),
            NyarReal::Infinity(v) => v.serialize(serializer),
        }
    }
}
