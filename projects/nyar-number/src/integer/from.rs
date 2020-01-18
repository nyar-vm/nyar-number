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
impl FromStr for NyarInteger {
    type Err = IntErrorKind;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match BigInt::from_str(s) {
            Ok(o) => Ok(NyarInteger::from(o)),
            Err(_) => Err(IntErrorKind::InvalidDigit),
        }
    }
}

impl From<u8> for NyarUnsigned {
    fn from(value: u8) -> Self {
        Self { _repr: BigUint::from(value) }
    }
}
impl From<u8> for NyarInteger {
    fn from(value: u8) -> Self {
        Self { sign: Default::default(), digits: Gc::new(NyarUnsigned::from(value)) }
    }
}
impl From<i8> for NyarInteger {
    fn from(value: i8) -> Self {
        NyarInteger::from(BigInt::from(value))
    }
}
impl From<u16> for NyarUnsigned {
    fn from(value: u16) -> Self {
        Self { _repr: BigUint::from(value) }
    }
}
impl From<u16> for NyarInteger {
    fn from(value: u16) -> Self {
        Self { sign: Default::default(), digits: Gc::new(NyarUnsigned::from(value)) }
    }
}
impl From<i16> for NyarInteger {
    fn from(value: i16) -> Self {
        NyarInteger::from(BigInt::from(value))
    }
}
impl From<u32> for NyarUnsigned {
    fn from(value: u32) -> Self {
        Self { _repr: BigUint::from(value) }
    }
}
impl From<u32> for NyarInteger {
    fn from(value: u32) -> Self {
        Self { sign: Default::default(), digits: Gc::new(NyarUnsigned::from(value)) }
    }
}
impl From<i32> for NyarInteger {
    fn from(value: i32) -> Self {
        NyarInteger::from(BigInt::from(value))
    }
}
impl From<u64> for NyarUnsigned {
    fn from(value: u64) -> Self {
        Self { _repr: BigUint::from(value) }
    }
}
impl From<u64> for NyarInteger {
    fn from(value: u64) -> Self {
        Self { sign: Default::default(), digits: Gc::new(NyarUnsigned::from(value)) }
    }
}
impl From<i64> for NyarInteger {
    fn from(value: i64) -> Self {
        NyarInteger::from(BigInt::from(value))
    }
}
impl From<u128> for NyarUnsigned {
    fn from(value: u128) -> Self {
        Self { _repr: BigUint::from(value) }
    }
}
impl From<u128> for NyarInteger {
    fn from(value: u128) -> Self {
        Self { sign: Default::default(), digits: Gc::new(NyarUnsigned::from(value)) }
    }
}
impl From<i128> for NyarInteger {
    fn from(value: i128) -> Self {
        NyarInteger::from(BigInt::from(value))
    }
}
impl From<usize> for NyarUnsigned {
    fn from(value: usize) -> Self {
        Self { _repr: BigUint::from(value) }
    }
}
impl From<usize> for NyarInteger {
    fn from(value: usize) -> Self {
        Self { sign: Default::default(), digits: Gc::new(NyarUnsigned::from(value)) }
    }
}
impl From<isize> for NyarInteger {
    fn from(value: isize) -> Self {
        NyarInteger::from(BigInt::from(value))
    }
}
impl From<BigUint> for NyarUnsigned {
    fn from(value: BigUint) -> Self {
        Self { _repr: value }
    }
}

impl From<BigUint> for NyarInteger {
    fn from(value: BigUint) -> Self {
        Self { sign: Default::default(), digits: Gc::new(NyarUnsigned { _repr: value }) }
    }
}

impl From<BigInt> for NyarInteger {
    fn from(value: BigInt) -> Self {
        let (lhs, rhs) = value.into_parts();
        Self { sign: NyarSign::from(lhs), digits: Gc::new(NyarUnsigned { _repr: rhs }) }
    }
}
