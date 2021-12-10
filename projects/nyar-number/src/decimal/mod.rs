use crate::{NyarDigits, NyarRational, One, Zero};
use bigdecimal::{BigDecimal, Num};
use num::{bigint::Sign, BigInt, FromPrimitive, Signed, ToPrimitive};
use nyar_error::NyarError;
use shredder::{
    marker::{GcDrop, GcSafe},
    Scan, Scanner,
};
use std::{
    fmt::{Debug, Display, Formatter},
    ops::{Add, Div, Mul, Neg, Rem, Sub},
    str::FromStr,
};

mod arith;
mod from;
mod into;

#[cfg(feature = "serde")]
mod der;
#[cfg(feature = "serde")]
mod ser;

/// A decimal fraction with dynamic precision
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct NyarDecimal {
    /// Indicates the sign bit of this number
    pub sign: Sign,
    /// Actual stored value
    pub digits: NyarDigits,
    /// Decimal point position
    pub scale: i64,
}

impl Display for NyarDecimal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.delegate(), f)
    }
}

unsafe impl GcSafe for NyarDecimal {}
unsafe impl GcDrop for NyarDecimal {}
unsafe impl Scan for NyarDecimal {
    fn scan(&self, scanner: &mut Scanner<'_>) {
        scanner.scan(&self.digits)
    }
}

impl NyarDecimal {
    pub(crate) fn delegate(&self) -> BigDecimal {
        let digits = BigInt::from_biguint(self.sign, self.digits.delegate());
        BigDecimal::new(digits, self.scale)
    }
}
