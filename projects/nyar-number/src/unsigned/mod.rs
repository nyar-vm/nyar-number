use num::{BigUint, One, Zero};
use shredder::{
    marker::{GcDrop, GcSafe},
    Gc, Scan, Scanner,
};
use std::{
    fmt::{Debug, Display, Formatter},
    num::IntErrorKind,
    ops::{Add, Mul},
    str::FromStr,
    sync::LazyLock,
};

mod arith;
#[cfg(feature = "serde")]
mod der;
mod from;
#[cfg(feature = "serde")]
mod ser;

pub(crate) static ZERO: LazyLock<Gc<NyarUnsigned>> = LazyLock::new(|| Gc::new(NyarUnsigned { _repr: BigUint::zero() }));
pub(crate) static ONE: LazyLock<Gc<NyarUnsigned>> = LazyLock::new(|| Gc::new(NyarUnsigned { _repr: BigUint::one() }));

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
