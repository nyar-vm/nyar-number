use num::{BigUint, One, Zero};
use shredder::{
    marker::{GcDrop, GcSafe},
    Scan, Scanner,
};
use std::{
    fmt::{Debug, Display, Formatter},
    num::IntErrorKind,
    ops::{Add, Div, Mul, Sub},
    str::FromStr,
};

mod arith;
mod from;

#[derive(Clone, Default, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct NyarUnsigned {
    pub(crate) _repr: BigUint,
}

unsafe impl GcSafe for NyarUnsigned {}

unsafe impl Scan for NyarUnsigned {
    fn scan(&self, _: &mut Scanner<'_>) {
        // no gc item inside
    }
}

unsafe impl GcDrop for NyarUnsigned {}

impl Debug for NyarUnsigned {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self._repr, f)
    }
}

impl Display for NyarUnsigned {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self._repr, f)
    }
}
