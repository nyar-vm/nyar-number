use super::*;
use nyar_error::NyarError;

impl FromStr for NyarDigits {
    type Err = NyarError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(BigUint::from_str(s)?.into())
    }
}

macro_rules! from_unsigned {
    ($($t:ty),*) => {
        $(
            impl From<$t> for NyarDigits {
                fn from(value: $t) -> Self {
                    Self::from(BigUint::from(value))
                }
            }
        )*
    };
}

from_unsigned![u8, u16, u32, u64, u128, usize];

impl From<BigUint> for NyarDigits {
    fn from(value: BigUint) -> Self {
        unsafe { Self { _repr: Gc::new(transmute::<BigUint, Vec<u64>>(value)) } }
    }
}
