use super::*;

impl FromStr for NyarUnsigned {
    type Err = IntErrorKind;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match BigUint::from_str(s) {
            Ok(o) => Ok(Self { _repr: o }),
            Err(_) => Err(IntErrorKind::InvalidDigit),
        }
    }
}

impl From<u8> for NyarUnsigned {
    fn from(value: u8) -> Self {
        Self { _repr: BigUint::from(value) }
    }
}

impl From<u16> for NyarUnsigned {
    fn from(value: u16) -> Self {
        Self { _repr: BigUint::from(value) }
    }
}
impl From<u32> for NyarUnsigned {
    fn from(value: u32) -> Self {
        Self { _repr: BigUint::from(value) }
    }
}
impl From<u64> for NyarUnsigned {
    fn from(value: u64) -> Self {
        Self { _repr: BigUint::from(value) }
    }
}
impl From<u128> for NyarUnsigned {
    fn from(value: u128) -> Self {
        Self { _repr: BigUint::from(value) }
    }
}
impl From<usize> for NyarUnsigned {
    fn from(value: usize) -> Self {
        Self { _repr: BigUint::from(value) }
    }
}
impl From<BigUint> for NyarUnsigned {
    fn from(value: BigUint) -> Self {
        Self { _repr: value }
    }
}