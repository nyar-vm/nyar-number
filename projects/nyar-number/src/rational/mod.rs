mod arith;
mod from;
mod into;

#[cfg(feature = "serde")]
mod der;
#[cfg(feature = "serde")]
mod ser;
use crate::{
    unsigned::{ONE, ZERO},
    NyarInteger, NyarUnsigned,
};
use num::{bigint::Sign, BigInt, BigRational, BigUint, Num, One, Signed, ToPrimitive, Zero};
use shredder::{
    marker::{GcDrop, GcSafe},
    Gc, Scan, Scanner,
};
use std::{
    clone::Clone,
    fmt::{Display, Formatter, Write},
    ops::{Add, Div, Mul, Neg, Rem, Sub},
    str::FromStr,
    sync::LazyLock,
};

pub(crate) static POSITIVE_INFINITY: LazyLock<Gc<NyarRational>> =
    LazyLock::new(|| Gc::new(NyarRational { sign: Sign::Plus, numerator: ONE.clone(), denominator: ZERO.clone() }));
pub(crate) static NEGATIVE_INFINITY: LazyLock<Gc<NyarRational>> =
    LazyLock::new(|| Gc::new(NyarRational { sign: Sign::Minus, numerator: ONE.clone(), denominator: ZERO.clone() }));

/// Infinite precision rational number type
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct NyarRational {
    /// Sign of rational numbers
    ///
    /// Used to distinguish between positive infinity and negative infinity when necessary
    pub sign: Sign,
    /// Numerator of rational numbers
    pub numerator: Gc<NyarUnsigned>,
    /// Denominator of rational numbers
    ///
    /// If the denominator is zero, it means infinity
    pub denominator: Gc<NyarUnsigned>,
}

impl Default for NyarRational {
    fn default() -> Self {
        Self { sign: Sign::NoSign, numerator: ZERO.clone(), denominator: ONE.clone() }
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
        let den = self.denominator.get();
        if den.is_zero() {
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
            if !self.denominator.get().is_one() {
                f.write_char('/')?;
                Display::fmt(&self.denominator, f)?
            }
        }
        Ok(())
    }
}

impl NyarRational {
    pub(crate) fn delegate(&self) -> BigRational {
        let num = BigInt::from_biguint(self.sign, self.numerator.get()._repr.clone());
        let den = BigInt::from_biguint(Sign::Plus, self.denominator.get()._repr.clone());
        BigRational::new(num, den)
    }

    /// Construct infinity with sign bit
    pub fn infinite(positive: bool) -> Gc<Self> {
        match positive {
            true => POSITIVE_INFINITY.clone(),
            false => NEGATIVE_INFINITY.clone(),
        }
    }

    /// Check if this represents the infinity
    pub fn is_infinite(&self) -> bool {
        // operations that require lock acquisition are placed later.
        self.denominator.get().is_zero()
    }

    /// Check if this represents the positive infinity
    pub fn is_positive_infinite(&self) -> bool {
        self.sign == Sign::Plus && self.is_positive()
    }

    /// Check if this represents the negative infinity
    pub fn is_negative_infinite(&self) -> bool {
        self.sign == Sign::Minus && self.is_positive()
    }
}
