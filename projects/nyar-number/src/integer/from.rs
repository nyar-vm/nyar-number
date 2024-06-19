use super::*;

impl FromStr for NyarInteger {
    type Err = SyntaxError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(BigInt::from_str(s)?.into())
    }
}

impl Num for NyarInteger {
    type FromStrRadixErr = SyntaxError;

    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        Ok(Self::from(BigInt::from_str_radix(str, radix)?))
    }
}
macro_rules! from_unsigned {
    ($($t:ty),*) => {
        $(
            impl From<$t> for NyarInteger {
                fn from(n: $t) -> Self {
                    if n.is_zero() { Self::zero() } else { Self { sign: Sign::Plus, digits: NyarDigits::from(n) } }
                }
            }

            impl From<&$t> for NyarInteger {
                fn from(n: &$t) -> Self {
                    if n.is_zero() { Self::zero() } else { Self { sign: Sign::Plus, digits: NyarDigits::from(n.clone()) } }
                }
            }
        )*
    };
}

from_unsigned![u8, u16, u32, u64, u128, usize, BigUint];

impl From<i8> for NyarInteger {
    fn from(value: i8) -> Self {
        NyarInteger::from(BigInt::from(value))
    }
}

impl From<i16> for NyarInteger {
    fn from(value: i16) -> Self {
        NyarInteger::from(BigInt::from(value))
    }
}

impl From<i32> for NyarInteger {
    fn from(value: i32) -> Self {
        NyarInteger::from(BigInt::from(value))
    }
}

impl From<i64> for NyarInteger {
    fn from(value: i64) -> Self {
        NyarInteger::from(BigInt::from(value))
    }
}

impl From<i128> for NyarInteger {
    fn from(value: i128) -> Self {
        NyarInteger::from(BigInt::from(value))
    }
}

impl From<isize> for NyarInteger {
    fn from(value: isize) -> Self {
        NyarInteger::from(BigInt::from(value))
    }
}

impl From<BigInt> for NyarInteger {
    fn from(value: BigInt) -> Self {
        let (lhs, rhs) = value.into_parts();
        Self { sign: lhs, digits: NyarDigits::from(rhs) }
    }
}
impl From<&BigInt> for NyarInteger {
    fn from(value: &BigInt) -> Self {
        let (lhs, rhs) = value.clone().into_parts();
        Self { sign: lhs, digits: NyarDigits::from(rhs) }
    }
}
