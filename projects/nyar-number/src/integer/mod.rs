use crate::unsigned::NyarDigits;
use num::{bigint::Sign, BigInt, BigUint, Num, One, Signed, ToPrimitive, Zero};
use nyar_error::{ SyntaxError};
use shredder::{
    marker::{GcDrop, GcSafe},
    Scan, Scanner,
};
use std::{
    fmt::{Debug, Display, Formatter, Write},
    ops::{Add, Div, Mul, Neg, Rem, Sub},
    str::FromStr,
};
use crate::NyarDecimal;

mod arith;
#[cfg(feature = "serde")]
mod der;
mod from;
mod into;
#[cfg(feature = "serde")]
mod ser;
#[cfg(feature = "jupyter-types")]
mod jupyter;

/// An infinite-precision signed integer
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct NyarInteger {
    /// Indicates the sign bit of this number
    pub sign: Sign,
    /// Actual stored value
    pub digits: NyarDigits,
}

impl Default for NyarInteger {
    fn default() -> Self {
        Self { sign: Sign::NoSign, digits: Default::default() }
    }
}


unsafe impl GcSafe for NyarInteger {}

unsafe impl Scan for NyarInteger {
    fn scan(&self, scanner: &mut Scanner<'_>) {
        scanner.scan(&self.digits)
    }
}

unsafe impl GcDrop for NyarInteger {}

impl Debug for NyarInteger {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NyarInteger").field("sign", &self.sign).field("value", &self.digits).finish()
    }
}

impl Display for NyarInteger {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Sign::Minus = self.sign {
            f.write_char('-')?
        }
        Display::fmt(&self.digits, f)
    }
}

impl NyarInteger {
    /// Create a new signed integer and box the unsigned value
    pub fn new<T>(sign: Sign, value: T) -> Self
    where
        T: Into<NyarDigits>,
    {
        Self { sign, digits: value.into() }
    }

    pub(crate) fn delegate(&self) -> BigInt {
        BigInt::from_biguint(self.sign, self.digits.delegate())
    }
}
