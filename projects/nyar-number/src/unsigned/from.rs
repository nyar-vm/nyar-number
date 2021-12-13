use super::*;
use num::Num;
use nyar_error::NyarError;

impl FromStr for NyarDigits {
    type Err = NyarError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(BigUint::from_str(s)?.into())
    }
}

impl Num for NyarDigits {
    type FromStrRadixErr = NyarError;

    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        Ok(BigUint::from_str_radix(str, radix)?.into())
    }
}
impl From<u8> for NyarDigits {
    fn from(value: u8) -> Self {
        Self::from(value as u64)
    }
}
impl From<u16> for NyarDigits {
    fn from(value: u16) -> Self {
        Self::from(value as u64)
    }
}
impl From<u32> for NyarDigits {
    fn from(value: u32) -> Self {
        Self::from(value as u64)
    }
}
impl From<u64> for NyarDigits {
    fn from(value: u64) -> Self {
        Self { _repr: Gc::new(vec![value]) }
    }
}
impl From<u128> for NyarDigits {
    fn from(value: u128) -> Self {
        Self::from(BigUint::from(value))
    }
}
impl From<usize> for NyarDigits {
    fn from(value: usize) -> Self {
        Self::from(value as u64)
    }
}

impl From<BigUint> for NyarDigits {
    fn from(value: BigUint) -> Self {
        unsafe { Self { _repr: Gc::new(transmute::<BigUint, Vec<u64>>(value)) } }
    }
}
