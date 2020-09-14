use crate::NyarReal;
use shredder::{
    marker::{GcDrop, GcSafe},
    Scan, Scanner,
};
use std::fmt::{Debug, Display, Formatter};

mod arith;
mod from;

#[cfg(feature = "serde")]
mod der;
#[cfg(feature = "serde")]
mod ser;

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct NyarComplex {
    pub re: NyarReal,
    pub im: NyarReal,
}

impl Display for NyarComplex {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.re, f)?;
        Display::fmt(&self.im, f)
    }
}

unsafe impl GcSafe for NyarComplex {}
unsafe impl GcDrop for NyarComplex {}
unsafe impl Scan for NyarComplex {
    fn scan(&self, scanner: &mut Scanner<'_>) {
        scanner.scan(&self.re);
        scanner.scan(&self.im);
    }
}
