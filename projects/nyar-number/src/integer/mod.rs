use super::*;
use dashu::integer::UBig;
use shredder::{marker::GcSafe, Scan, Scanner};
// use shredder::{Gc, Scanner};
// use valkyrie_error::third_party::UBig;

pub struct ValkyrieInteger {
    _repr: UBig,
}

unsafe impl GcSafe for ValkyrieInteger {}

unsafe impl Scan for ValkyrieInteger {
    fn scan(&self, _: &mut Scanner<'_>) {
        // no gc item inside
    }
}
