use super::*;

impl Num for NyarDecimal {
    type FromStrRadixErr = NyarError;

    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        Ok(BigDecimal::from_str_radix(str, radix)?.into())
    }
}

impl FromStr for NyarDecimal {
    type Err = NyarError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(BigDecimal::from_str_radix(s, 10)?.into())
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
        Self { sign, digits: Gc::new(NyarUnsigned::from(digits)), scale }
    }
}

impl TryFrom<f32> for NyarDecimal {
    type Error = NyarError;

    fn try_from(value: f32) -> Result<Self, Self::Error> {
        Ok(NyarDecimal::from(BigDecimal::try_from(value)?))
    }
}

impl TryFrom<f64> for NyarDecimal {
    type Error = NyarError;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        Ok(NyarDecimal::from(BigDecimal::try_from(value)?))
    }
}
