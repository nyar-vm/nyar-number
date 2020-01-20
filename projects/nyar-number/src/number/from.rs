use super::*;
use num::{BigInt, BigUint, Num};
impl Num for NyarNumber {
    type FromStrRadixErr = ();

    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        todo!()
    }
}

impl From<NyarRational> for NyarNumber {
    fn from(value: NyarRational) -> Self {
        Self::Rational(value)
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
