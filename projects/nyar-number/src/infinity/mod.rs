use bigdecimal::num_bigint::Sign;
use num::traits::NumOps;
use shredder::{marker::GcSafe, Scan, Scanner};
use std::{
    fmt::{Debug, Display, Formatter},
    ops::{Add, Div, Mul, Neg, Rem, Sub},
};
use Sign::{Minus, NoSign, Plus};

mod ser;

mod der;

mod arith;

mod into;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct NyarInfinity {
    sign: Sign,
}

impl Display for NyarInfinity {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.sign {
            Plus => f.write_str("+∞"),
            Minus => f.write_str("-∞"),
            NoSign => f.write_str("∾"),
        }
    }
}
impl Debug for NyarInfinity {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.sign {
            Plus => f.write_str("PositiveInfinity"),
            Minus => f.write_str("NegativeInfinity"),
            NoSign => f.write_str("Indeterminate"),
        }
    }
}

unsafe impl GcSafe for NyarInfinity {}

unsafe impl Scan for NyarInfinity {
    fn scan(&self, _: &mut Scanner<'_>) {}
}

impl NyarInfinity {
    pub const POSITIVE_INFINITY: Self = Self { sign: Plus };
    pub const NEGATIVE_INFINITY: Self = Self { sign: Minus };
    pub const INDETERMINATE: Self = Self { sign: NoSign };

    pub fn is_positive(&self) -> bool {
        matches!(self.sign, Sign::Plus)
    }
    pub fn is_negative(&self) -> bool {
        matches!(self.sign, Sign::Minus)
    }
    pub fn is_indeterminate(&self) -> bool {
        matches!(self.sign, Sign::NoSign)
    }
}
