use bigdecimal::num_bigint::Sign;
use shredder::{marker::GcSafe, Scan, Scanner};
use std::ops::Add;
use Sign::{Minus, NoSign, Plus};
mod ser;

mod der;

mod arith;

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct NyarInfinity {
    sign: Sign,
}

unsafe impl GcSafe for NyarInfinity {}

unsafe impl Scan for NyarInfinity {
    fn scan(&self, _: &mut Scanner<'_>) {}
}

impl NyarInfinity {
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
