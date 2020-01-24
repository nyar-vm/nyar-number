use super::*;
use crate::utils::NyarNumberError;

impl Num for NyarDecimal {
    type FromStrRadixErr = ParseBigDecimalError;

    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        BigDecimal::from_str_radix(str, radix).map(|v| v.into())
    }
}

impl FromStr for NyarDecimal {
    type Err = ParseBigDecimalError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        BigDecimal::from_str_radix(s, 10).map(|v| v.into())
    }
}
impl From<NyarRational> for NyarDecimal {
    fn from(value: NyarRational) -> Self {
        value.delegate().to_f64().and_then(BigDecimal::from_f64).unwrap_or_default().into()
    }
}

impl From<BigDecimal> for NyarDecimal {
    fn from(value: BigDecimal) -> Self {
        let (value, scale) = value.into_bigint_and_exponent();
        let (sign, digits) = value.into_parts();
        Self { sign, digits: Gc::new(NyarUnsigned { _repr: digits }), scale }
    }
}

impl TryFrom<f32> for NyarDecimal {
    type Error = NyarNumberError;

    fn try_from(value: f32) -> Result<Self, Self::Error> {
        Ok(NyarDecimal::from(BigDecimal::try_from(value)?))
    }
}

impl TryFrom<f64> for NyarDecimal {
    type Error = NyarNumberError;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        Ok(NyarDecimal::from(BigDecimal::try_from(value)?))
    }
}
