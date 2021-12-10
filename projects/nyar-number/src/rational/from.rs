use super::*;
use num::rational::ParseRatioError;

impl Num for NyarRational {
    type FromStrRadixErr = ParseRatioError;

    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        BigRational::from_str_radix(str, radix).map(|o| NyarRational::from(o))
    }
}

impl FromStr for NyarRational {
    type Err = ParseRatioError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        BigRational::from_str_radix(s, 10).map(|o| NyarRational::from(o))
    }
}

impl From<BigRational> for NyarRational {
    fn from(value: BigRational) -> Self {
        let (num, den) = value.into();
        let sign = num.sign().mul(den.sign());
        let num = NyarInteger::from(num);
        let den = NyarInteger::from(den);
        NyarRational { sign, numerator: num.digits, denominator: den.digits }
    }
}
impl From<NyarInteger> for NyarRational {
    fn from(value: NyarInteger) -> Self {
        Self { sign: value.sign, numerator: value.digits, denominator: NyarDigits::one() }
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
