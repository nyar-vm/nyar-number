use num::BigUint;
use shredder::{marker::GcSafe, Scan, Scanner};
use std::{
    fmt::{Debug, Display, Formatter},
    num::IntErrorKind,
    ops::{Add, Mul},
    str::FromStr,
};

mod arith;
mod from;

#[derive(Clone, Default, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct NyarInteger {
    _repr: BigUint,
}

unsafe impl GcSafe for NyarInteger {}

unsafe impl Scan for NyarInteger {
    fn scan(&self, _: &mut Scanner<'_>) {
        // no gc item inside
    }
}

impl Debug for NyarInteger {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self._repr, f)
    }
}

impl Display for NyarInteger {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self._repr, f)
    }
}
