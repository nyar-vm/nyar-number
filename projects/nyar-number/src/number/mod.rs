use super::*;
use shredder::{Gc, Scanner};
use valkyrie_error::third_party::UBig;

pub struct ValkyrieInteger {
    _repr: UBig,
}
pub struct ValkyrieDecimal {
    _repr: FBig,
}

#[derive(Clone, Debug)]
pub enum ValkyrieNumber {
    Integer { negative: bool, represent: Gc<ValkyrieInteger> },
    Decimal { represent: Gc<ValkyrieDecimal> },
    Rational { negative: bool, numerator: Gc<ValkyrieInteger>, denominator: Gc<ValkyrieInteger> },
}

unsafe impl GcSafe for ValkyrieInteger {}

unsafe impl Scan for ValkyrieInteger {
    fn scan(&self, scanner: &mut Scanner<'_>) {
        match self {
            ValkyrieNumber::Integer { .. } => {}
            ValkyrieNumber::Decimal(_) => {}
            ValkyrieNumber::Rational { .. } => {}
        }
    }
}

//
// impl From<u8> for ValkyrieNumber {
//     fn from(value: u8) -> Self {
//         Self::Integer { negative: false, represent: () }
//     }
// }
