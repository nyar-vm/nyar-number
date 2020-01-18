use crate::integer::NyarInteger;

use shredder::{Gc, Scan};

// #[derive(Clone, Debug, Scan)]
pub enum NyarNumber {
    /// A signed integer
    Integer {
        ///
        positive: bool,
        represent: Gc<NyarInteger>,
    },
    /// A signed rational number
    Rational {
        positive: bool,
        numerator: Gc<NyarInteger>,
        denominator: Gc<NyarInteger>,
    },
    /// A signed decimal number
    Decimal {
        represent: Gc<NyarInteger>,
    },
    Complex {
        re: Gc<NyarNumber>,
        im: Gc<NyarNumber>,
    },
}
