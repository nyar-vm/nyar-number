use crate::{
    unsigned::{ONE, ZERO},
    NyarRational, NyarUnsigned, One, Zero,
};
use bigdecimal::{BigDecimal, Num, ParseBigDecimalError};
use num::{bigint::Sign, BigInt, FromPrimitive, Signed, ToPrimitive};
use shredder::{
    marker::{GcDrop, GcSafe},
    Gc, Scan, Scanner,
};
use std::{
    fmt::{Debug, Display, Formatter},
    ops::{Add, Div, Mul, Neg, Rem, Sub},
    str::FromStr,
};

mod arith;
mod from;

#[cfg(feature = "serde")]
mod der;
#[cfg(feature = "serde")]
mod ser;

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct NyarDecimal {
    pub sign: Sign,
    pub digits: Gc<NyarUnsigned>,
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
        let digits = BigInt::from_biguint(self.sign, self.digits.get()._repr.clone());
        BigDecimal::new(digits, self.scale)
    }
}
