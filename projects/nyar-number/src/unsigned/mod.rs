use num::{BigUint, One, ToPrimitive, Zero};
use shredder::{
    marker::{GcDrop, GcSafe},
    Gc, Scan, Scanner,
};
use std::{
    fmt::{Debug, Display, Formatter},
    mem::transmute,
    ops::{Add, Mul},
    str::FromStr,
    sync::LazyLock,
};

mod arith;
#[cfg(feature = "serde")]
mod der;
mod from;
mod into;
#[cfg(feature = "serde")]
mod ser;

/// The underlying representation of all infinite-precision numbers
#[derive(Clone, Default, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct NyarDigits {
    pub(crate) _repr: Gc<Vec<u64>>,
}

unsafe impl GcSafe for NyarDigits {}

unsafe impl Scan for NyarDigits {
    fn scan(&self, _: &mut Scanner<'_>) {
        // no gc item inside
    }
}

unsafe impl GcDrop for NyarDigits {}

impl Debug for NyarDigits {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let value = self.delegate();
        write!(f, "{} ({:p})", value, self._repr)
    }
}

impl Display for NyarDigits {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let value = self.delegate();
        write!(f, "{}", value)
    }
}

impl NyarDigits {
    pub(crate) fn delegate(&self) -> BigUint {
        let bytes = self._repr.get().clone();
        unsafe { transmute::<Vec<u64>, BigUint>(bytes) }
    }
}
