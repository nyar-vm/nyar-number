use crate::{utils::NyarNumberError, NyarDecimal, NyarInteger, NyarRational, One, Zero};
use num::{BigInt, BigUint, Num, Signed};
use shredder::Scan;
use std::{
    fmt::{Debug, Display, Formatter},
    ops::{Add, Div, Mul, Neg, Rem, Sub},
};

mod arith;
#[cfg(feature = "serde")]
mod der;
mod from;
#[cfg(feature = "serde")]
mod ser;

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Scan)]
pub enum NyarNumber {
    /// A signed rational number
    Rational(NyarRational),
    /// A signed decimal number
    Decimal(NyarDecimal),
    Complex(NyarInteger),
}

impl Display for NyarNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            NyarNumber::Rational(v) => Display::fmt(v, f),
            NyarNumber::Decimal(v) => Display::fmt(v, f),
            NyarNumber::Complex(v) => Display::fmt(v, f),
        }
    }
}

impl NyarNumber {
    pub fn parse_integer(input: &str) -> Result<Self, NyarNumberError> {
        let int = NyarInteger::from_str_radix(input, 10)?;
        Ok(Self::Rational(NyarRational::from(int)))
    }
    pub fn parse_integer_radix(input: &str, radix: u32) -> Result<Self, NyarNumberError> {
        let int = NyarInteger::from_str_radix(input, radix)?;
        Ok(Self::Rational(NyarRational::from(int)))
    }
}
