use super::*;

impl Num for NyarReal {
    type FromStrRadixErr = NyarNumberError;

    fn from_str_radix(input: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        if input.contains('.') {
            return Ok(Self::Decimal(NyarDecimal::from_str_radix(input, radix)?));
        }
        if input.contains('/') {
            return Ok(Self::Rational(NyarRational::from_str_radix(input, radix)?));
        }
        Ok(Self::Rational(BigInt::from_str_radix(input, radix)?.into()))
    }
}

impl From<NyarRational> for NyarReal {
    fn from(value: NyarRational) -> Self {
        Self::Rational(value)
    }
}

impl TryFrom<f32> for NyarReal {
    type Error = NyarNumberError;

    fn try_from(value: f32) -> Result<Self, Self::Error> {
        Ok(Self::Decimal(NyarDecimal::try_from(value)?))
    }
}
impl TryFrom<f64> for NyarReal {
    type Error = NyarNumberError;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        Ok(Self::Decimal(NyarDecimal::try_from(value)?))
    }
}

macro_rules! impl_integer {
    ($($ty:ty),*) => {$(
        impl From<$ty> for NyarReal {
            fn from(value: $ty) -> Self {
                Self::Rational(NyarRational::from(value))
            }
            }
        )*};
}

impl_integer![u8, u16, u32, u64, u128, usize, BigUint, &BigUint];
impl_integer![i8, i16, i32, i64, i128, isize, BigInt, &BigInt];
