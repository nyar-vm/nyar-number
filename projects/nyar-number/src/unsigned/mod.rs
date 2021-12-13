use num::{traits::ToBytes, BigUint, CheckedSub, Integer, One, ToPrimitive, Zero};
use shredder::{
    marker::{GcDrop, GcSafe},
    Gc, Scan, Scanner,
};
use std::{
    cmp::Ordering,
    fmt::{Debug, Display, Formatter},
    mem::transmute,
    ops::{Add, Div, Mul, Rem, Sub},
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
    /// Starting from the left represents the first digit, and excess zeros on the right can be removed.
    pub(crate) _repr: Gc<Vec<u64>>,
}

unsafe impl GcSafe for NyarDigits {}

unsafe impl Scan for NyarDigits {
    fn scan(&self, scanner: &mut Scanner<'_>) {
        self._repr.scan(scanner)
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
    /// Create new unsigned from digits
    pub fn new(v: Vec<u64>) -> Self {
        if v.is_empty() {
            return Self::zero();
        }
        Self { _repr: Gc::new(v) }
    }
    pub(crate) fn delegate(&self) -> BigUint {
        let bytes = self._repr.get().clone();
        unsafe { transmute::<Vec<u64>, BigUint>(bytes) }
    }
    /// Convert from number
    ///
    /// # Arguments
    ///
    /// * `v`:
    ///
    /// returns: NyarDigits
    ///
    /// # Examples
    ///
    /// ```
    /// ```
    pub fn from_bytes(v: &[u8]) -> Self {
        Self::from(BigUint::from_bytes_le(v))
    }
    /// Convert to bytes
    ///
    /// # Arguments
    ///
    /// * `v`:
    ///
    /// returns: NyarDigits
    ///
    /// # Examples
    ///
    /// ```
    /// ```
    pub fn into_bytes(self) -> Vec<u8> {
        self.delegate().to_le_bytes()
    }
}
