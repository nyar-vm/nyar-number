use crate::sign::NyarSign;
use num::{BigInt, BigUint, One, Signed, Zero};
use shredder::{
    marker::{GcDrop, GcSafe},
    Gc, Scan, Scanner,
};
use std::{
    fmt::{Debug, Display, Formatter, Write},
    num::IntErrorKind,
    ops::{Add, Div, Mul, Sub},
    str::FromStr,
};

mod arith;
mod from;

#[derive(Clone, Default, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct NyarUnsigned {
    _repr: BigUint,
}

#[derive(Clone, Debug, Default, Ord, PartialOrd, Eq, PartialEq, Hash, Scan)]
pub struct NyarInteger {
    pub sign: NyarSign,
    pub digits: Gc<NyarUnsigned>,
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

impl Display for NyarInteger {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.sign {
            NyarSign::Positive => {}
            NyarSign::Negative => f.write_char('-')?,
        }
        Display::fmt(&self.digits, f)
    }
}
