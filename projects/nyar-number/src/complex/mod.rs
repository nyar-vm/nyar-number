use crate::{
    unsigned::{ONE, ZERO},
    NyarNumber, NyarRational, NyarReal, NyarUnsigned, One, Zero,
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
pub struct NyarComplex {
    pub re: NyarReal,
    pub im: NyarReal,
}

impl Display for NyarComplex {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.delegate(), f)
    }
}

unsafe impl GcSafe for NyarComplex {}
unsafe impl GcDrop for NyarComplex {}
unsafe impl Scan for NyarComplex {
    fn scan(&self, scanner: &mut Scanner<'_>) {
        scanner.scan(&self.digits)
    }
}

impl NyarComplex {
    pub(crate) fn delegate(&self) -> BigDecimal {
        let digits = BigInt::from_biguint(self.sign, self.digits.get()._repr.clone());
        BigDecimal::new(digits, self.scale)
    }
}
