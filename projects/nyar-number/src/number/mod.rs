use crate::{utils::NyarNumberError, NyarDecimal, NyarInteger, NyarRational, One, Zero};
use num::{BigInt, BigUint, Num, Signed};
use shredder::Scan;
use std::{
    fmt::{Debug, Display, Formatter},
    ops::{Add, Div, Mul, Neg, Rem, Sub},
    str::FromStr,
};

mod arith;
#[cfg(feature = "serde")]
mod der;
mod from;
mod into;
#[cfg(feature = "serde")]
mod ser;

/// A real number, which can be a fraction with infinite precision or a decimal with dynamic progress
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Scan)]
pub enum NyarReal {
    /// A signed rational number
    Rational(NyarRational),
    /// A signed decimal number
    Decimal(NyarDecimal),
}

impl Default for NyarReal {
    fn default() -> Self {
        Self::Rational(NyarRational::default())
    }
}

impl Display for NyarReal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            NyarReal::Rational(v) => Display::fmt(v, f),
            NyarReal::Decimal(v) => Display::fmt(v, f),
        }
    }
}

impl NyarReal {
    /// Parse string into decimal integer
    ///
    /// # Arguments
    ///
    /// * `input`:
    ///
    /// returns: Result<NyarReal, NyarNumberError>
    ///
    /// # Examples
    ///
    /// ```
    /// // nothing
    /// ```
    pub fn parse_integer(input: &str) -> Result<Self, NyarNumberError> {
        let int = NyarInteger::from_str_radix(input, 10)?;
        Ok(Self::Rational(NyarRational::from(int)))
    }
    /// Parse string into decimal fraction
    ///
    /// # Arguments
    ///
    /// * `input`:
    ///
    /// returns: Result<NyarReal, NyarNumberError>
    ///
    /// # Examples
    ///
    /// ```
    /// // nothing
    /// ```
    pub fn parse_decimal(input: &str) -> Result<Self, NyarNumberError> {
        Ok(Self::Decimal(NyarDecimal::from_str_radix(input, 10)?))
    }
    /// Parse string into decimal fraction
    ///
    /// # Arguments
    ///
    /// * `input`:
    /// * `radix`:
    ///
    /// returns: Result<NyarReal, NyarNumberError>
    ///
    /// # Examples
    ///
    /// ```
    /// // nothing
    /// ```
    pub fn parse_integer_radix(input: &str, radix: u32) -> Result<Self, NyarNumberError> {
        let int = NyarInteger::from_str_radix(input, radix)?;
        Ok(Self::Rational(NyarRational::from(int)))
    }
    /// Parse string into decimal fraction
    ///
    /// # Arguments
    ///
    /// * `input`:
    /// * `radix`:
    ///
    /// returns: Result<NyarReal, NyarNumberError>
    ///
    /// # Examples
    ///
    /// ```
    /// // nothing
    /// ```
    pub fn parse_decimal_radix(input: &str, radix: u32) -> Result<Self, NyarNumberError> {
        Ok(Self::Decimal(NyarDecimal::from_str_radix(input, radix)?))
    }
}
