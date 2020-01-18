use super::*;

impl FromStr for NyarInteger {
    type Err = IntErrorKind;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match UBig::from_str(s) {
            Ok(o) => Ok(Self { _repr: o }),
            Err(e) => Err(match e {
                ParseError::NoDigits => IntErrorKind::Empty,
                ParseError::InvalidDigit => IntErrorKind::InvalidDigit,
                ParseError::UnsupportedRadix => IntErrorKind::InvalidDigit,
                ParseError::InconsistentRadix => IntErrorKind::InvalidDigit,
            }),
        }
    }
}

impl From<u8> for NyarInteger {
    fn from(value: u8) -> Self {
        Self { _repr: UBig::from(value) }
    }
}

impl From<u16> for NyarInteger {
    fn from(value: u16) -> Self {
        Self { _repr: UBig::from(value) }
    }
}

impl From<u32> for NyarInteger {
    fn from(value: u32) -> Self {
        Self { _repr: UBig::from(value) }
    }
}
impl From<u64> for NyarInteger {
    fn from(value: u64) -> Self {
        Self { _repr: UBig::from(value) }
    }
}
impl From<u128> for NyarInteger {
    fn from(value: u128) -> Self {
        Self { _repr: UBig::from(value) }
    }
}
impl From<usize> for NyarInteger {
    fn from(value: usize) -> Self {
        Self { _repr: UBig::from(value) }
    }
}
