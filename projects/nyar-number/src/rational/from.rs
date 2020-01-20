use super::*;
use crate::NyarNumber;
use num::{BigInt, BigUint};
impl From<BigRational> for NyarRational {
    fn from(value: BigRational) -> Self {
        let (num, den) = value.into();
        let sign = num.sign().mul(den.sign());
        let num = NyarInteger::from(num);
        let den = NyarInteger::from(den);
        NyarRational { sign, numerator: num.digits, denominator: den.digits }
    }
}

impl FromStr for NyarRational {
    type Err = IntErrorKind;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match BigRational::from_str(s) {
            Ok(o) => Ok(NyarRational::from(o)),
            Err(_) => Err(IntErrorKind::InvalidDigit),
        }
    }
}

impl From<NyarInteger> for NyarRational {
    fn from(value: NyarInteger) -> Self {
        Self { sign: value.sign, numerator: value.digits, denominator: ONE.clone() }
    }
}

macro_rules! impl_integer {
    ($($ty:ty),*) => {$(
        impl From<$ty> for NyarRational {
            fn from(value: $ty) -> Self {
                NyarRational::from(NyarInteger::from(value))
            }
            }
        )*};
}

impl_integer![u8, u16, u32, u64, u128, usize, BigUint, &BigUint];
impl_integer![i8, i16, i32, i64, i128, isize, BigInt, &BigInt];
