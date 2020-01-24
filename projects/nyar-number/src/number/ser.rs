use crate::NyarNumber;
use serde::{Serialize, Serializer};

impl Serialize for NyarNumber {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            NyarNumber::Rational(v) => v.serialize(serializer),
            NyarNumber::Decimal(v) => v.serialize(serializer),
            NyarNumber::Complex(v) => v.serialize(serializer),
        }
    }
}
