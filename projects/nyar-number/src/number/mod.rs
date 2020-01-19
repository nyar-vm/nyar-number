use crate::{NyarInteger, NyarRational, One, Zero};
use num::{bigint::ParseBigIntError, Num, Signed};
use shredder::Scan;
use std::{
    fmt::{Debug, Display, Formatter},
    ops::{Add, Div, Mul, Neg, Rem, Sub},
};

mod arith;
mod from;

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Scan)]
pub enum NyarNumber {
    /// A signed integer
    Integer(NyarInteger),
    /// A signed rational number
    Rational(NyarRational),
    /// A signed decimal number
    Decimal(NyarInteger),
    Complex(NyarInteger),
}

impl Display for NyarNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            NyarNumber::Integer(v) => Display::fmt(v, f),
            NyarNumber::Rational(v) => Display::fmt(v, f),
            NyarNumber::Decimal(v) => Display::fmt(v, f),
            NyarNumber::Complex(v) => Display::fmt(v, f),
        }
    }
}

impl NyarNumber {
    pub fn parse_integer(input: &str) -> Result<Self, ParseBigIntError> {
        let int = NyarInteger::from_str_radix(input, 10)?;
        Ok(Self::Integer(int))
    }
    pub fn parse_integer_radix(input: &str, radix: u32) -> Result<Self, ParseBigIntError> {
        let int = NyarInteger::from_str_radix(input, radix)?;
        Ok(Self::Integer(int))
    }
}
