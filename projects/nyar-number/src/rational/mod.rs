mod arith;
mod from;
mod into;

#[cfg(feature = "serde")]
mod der;
#[cfg(feature = "serde")]
mod ser;
use crate::{NyarDigits, NyarInteger};
use num::{bigint::Sign, BigInt, BigRational, BigUint, CheckedDiv, Num, One, Signed, ToPrimitive, Zero};
use shredder::{
    marker::{GcDrop, GcSafe},
    Scan, Scanner,
};
use std::{
    clone::Clone,
    fmt::{Debug, Display, Formatter, Write},
    ops::{Add, Div, Mul, Neg, Rem, Sub},
    str::FromStr,
};
/// Infinite precision rational number type
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct NyarRational {
    /// Sign of rational numbers
    ///
    /// Used to distinguish between positive infinity and negative infinity when necessary
    pub sign: Sign,
    /// Numerator of rational numbers
    pub numerator: NyarDigits,
    /// Denominator of rational numbers
    ///
    /// If the denominator is zero, it means infinity
    pub denominator: NyarDigits,
}

impl Default for NyarRational {
    fn default() -> Self {
        Self::zero()
    }
}

unsafe impl GcSafe for NyarRational {}

unsafe impl Scan for NyarRational {
    fn scan(&self, scanner: &mut Scanner<'_>) {
        scanner.scan(&self.numerator);
        scanner.scan(&self.denominator);
    }
}
unsafe impl GcDrop for NyarRational {}

impl Display for NyarRational {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.denominator.is_zero() {
            match self.sign {
                Sign::Minus => f.write_str("-∞")?,
                Sign::NoSign => f.write_str("∞")?,
                Sign::Plus => f.write_str("+∞")?,
            }
        }
        else {
            match self.sign {
                Sign::Minus => f.write_char('-')?,
                Sign::NoSign => {}
                Sign::Plus => {}
            }
            Display::fmt(&self.numerator, f)?;
            if !self.denominator.is_one() {
                f.write_char('/')?;
                Display::fmt(&self.denominator, f)?
            }
        }
        Ok(())
    }
}

impl Debug for NyarRational {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Rational")
            .field("sign", &self.sign)
            .field("numerator", &self.numerator)
            .field("denominator", &self.denominator)
            .finish()
    }
}

impl NyarRational {
    pub(crate) fn delegate(&self) -> BigRational {
        let num = BigInt::from_biguint(self.sign, self.numerator.delegate().clone());
        let den = BigInt::from_biguint(Sign::Plus, self.denominator.delegate().clone());
        BigRational::new(num, den)
    }

    /// Check if this represents the infinity
    pub fn is_infinite(&self) -> bool {
        // operations that require lock acquisition are placed later.
        self.denominator.is_zero()
    }
}
