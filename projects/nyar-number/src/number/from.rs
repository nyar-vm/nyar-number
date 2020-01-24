use super::*;

impl Num for NyarNumber {
    type FromStrRadixErr = NyarNumberError;

    fn from_str_radix(input: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        if input.contains('.') {
            return Ok(Self::Decimal(NyarDecimal::from_str_radix(input, radix)?));
        }
        todo!()
    }
}

impl From<NyarRational> for NyarNumber {
    fn from(value: NyarRational) -> Self {
        Self::Rational(value)
    }
}

impl TryFrom<f32> for NyarNumber {
    type Error = NyarNumberError;

    fn try_from(value: f32) -> Result<Self, Self::Error> {
        Ok(Self::Decimal(NyarDecimal::try_from(value)?))
    }
}
impl TryFrom<f64> for NyarNumber {
    type Error = NyarNumberError;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        Ok(Self::Decimal(NyarDecimal::try_from(value)?))
    }
}

macro_rules! impl_integer {
    ($($ty:ty),*) => {$(
        impl From<$ty> for NyarNumber {
            fn from(value: $ty) -> Self {
                Self::Rational(NyarRational::from(value))
            }
            }
        )*};
}

impl_integer![u8, u16, u32, u64, u128, usize, BigUint, &BigUint];
impl_integer![i8, i16, i32, i64, i128, isize, BigInt, &BigInt];
