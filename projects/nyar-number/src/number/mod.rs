use crate::{infinity::NyarInfinity, NyarDecimal, NyarInteger, NyarRational, One, Zero};
use num::{bigint::Sign, BigInt, BigUint, Num, Signed};
use nyar_error::NyarError;
use shredder::{marker::GcSafe, Scan, Scanner};
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
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum NyarReal {
    Infinity(NyarInfinity),
    /// A signed rational number
    Rational(NyarRational),
    /// A signed decimal number
    Decimal(NyarDecimal),
}

unsafe impl GcSafe for NyarReal {}

unsafe impl Scan for NyarReal {
    fn scan(&self, scanner: &mut Scanner<'_>) {
        match self {
            NyarReal::Infinity(_) => {}
            NyarReal::Rational(v) => v.scan(scanner),
            NyarReal::Decimal(v) => v.scan(scanner),
        }
    }
}

impl Default for NyarReal {
    fn default() -> Self {
        Self::Rational(NyarRational::default())
    }
}

impl Display for NyarReal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Rational(v) => Display::fmt(v, f),
            Self::Decimal(v) => Display::fmt(v, f),
            Self::Infinity(v) => Display::fmt(v, f),
        }
    }
}
impl Debug for NyarReal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Rational(v) => Debug::fmt(v, f),
            Self::Decimal(v) => Debug::fmt(v, f),
            Self::Infinity(v) => Debug::fmt(v, f),
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
    pub fn parse_integer(input: &str) -> Result<Self, NyarError> {
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
    pub fn parse_decimal(input: &str) -> Result<Self, NyarError> {
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
    pub fn parse_integer_radix(input: &str, radix: u32) -> Result<Self, NyarError> {
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
    pub fn parse_decimal_radix(input: &str, radix: u32) -> Result<Self, NyarError> {
        Ok(Self::Decimal(NyarDecimal::from_str_radix(input, radix)?))
    }

    /// Create a infinity number
    ///
    /// # Arguments
    ///
    /// * `sign`:
    ///
    /// returns: ()
    ///
    /// # Examples
    ///
    /// ```
    /// // nothing
    /// ```
    pub fn infinity(sign: Sign) -> Self {
        Self::Infinity(match sign {
            Sign::Plus => NyarInfinity::POSITIVE_INFINITY,
            Sign::Minus => NyarInfinity::NEGATIVE_INFINITY,
            Sign::NoSign => NyarInfinity::INDETERMINATE,
        })
    }
}
