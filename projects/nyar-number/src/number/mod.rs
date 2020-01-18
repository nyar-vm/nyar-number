use crate::integer::ValkyrieInteger;
use shredder::{Gc, Scan};

#[derive(Clone, Debug, Scan)]
pub enum ValkyrieNumber {
    /// A signed integer
    Integer {
        negative: bool,
        represent: Gc<ValkyrieInteger>,
    },
    /// A signed rational number
    Rational {
        negative: bool,
        numerator: Gc<ValkyrieInteger>,
        denominator: Gc<ValkyrieInteger>,
    },
    /// A signed decimal number
    Decimal {
        represent: Gc<ValkyrieInteger>,
    },
    Complex {
        re: Gc<ValkyrieNumber>,
        im: Gc<ValkyrieNumber>,
    },
}
