mod from;

use crate::{sign::NyarSign, NyarInteger, NyarUnsigned};
use num::{
    rational::{ParseRatioError, Ratio},
    BigInt, BigRational,
};
use shredder::Gc;
use std::{
    num::{IntErrorKind, ParseIntError},
    str::FromStr,
};

#[derive(Clone, Default, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct NyarRational {
    pub sign: NyarSign,
    pub numerator: Gc<NyarUnsigned>,
    pub denominator: Gc<NyarUnsigned>,
}
