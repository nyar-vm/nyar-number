use crate::unsigned::NyarUnsigned;
use num::{bigint::Sign, BigInt, BigUint, Num, One, Signed, Zero};
use shredder::{
    marker::{GcDrop, GcSafe},
    Gc, Scan, Scanner,
};
use std::{
    fmt::{Debug, Display, Formatter, Write},
    num::IntErrorKind,
    ops::{Add, Div, Mul, Neg, Rem, Sub},
    str::FromStr,
};

mod arith;
mod from;

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct NyarInteger {
    pub sign: Sign,
    pub digits: Gc<NyarUnsigned>,
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
        T: Into<NyarUnsigned>,
    {
        Self { sign, digits: Gc::new(value.into()) }
    }

    pub(crate) fn wrapped(&self) -> BigInt {
        BigInt::from_biguint(self.sign, self.digits.get()._repr.clone())
    }
}
