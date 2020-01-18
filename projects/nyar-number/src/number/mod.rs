use crate::integer::NyarUnsigned;

use shredder::{Gc, Scan};

#[derive(Clone, Debug, Scan)]
pub enum NyarNumber {
    /// A signed integer
    Integer {
        ///
        positive: bool,
        represent: Gc<NyarUnsigned>,
    },
    /// A signed rational number
    Rational {
        positive: bool,
        numerator: Gc<NyarUnsigned>,
        denominator: Gc<NyarUnsigned>,
    },
    /// A signed decimal number
    Decimal {
        represent: Gc<NyarUnsigned>,
    },
    Complex {
        re: Gc<NyarNumber>,
        im: Gc<NyarNumber>,
    },
}
