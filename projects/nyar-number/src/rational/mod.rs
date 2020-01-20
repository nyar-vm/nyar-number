mod arith;
mod from;

use crate::{unsigned::ONE, NyarInteger, NyarUnsigned};
use num::{bigint::Sign, BigRational, One};
use shredder::{
    marker::{GcDrop, GcSafe},
    Gc, Scan, Scanner,
};
use std::{
    fmt::{Display, Formatter, Write},
    num::IntErrorKind,
    ops::Mul,
    str::FromStr,
};
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct NyarRational {
    pub sign: Sign,
    pub numerator: Gc<NyarUnsigned>,
    pub denominator: Gc<NyarUnsigned>,
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
        Ok(())
    }
}
