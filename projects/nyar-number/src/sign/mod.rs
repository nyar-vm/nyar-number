use num::bigint::Sign;
use shredder::Scan;

#[repr(u8)]
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Scan)]
pub enum NyarSign {
    Positive,
    Negative,
}

impl Default for NyarSign {
    fn default() -> Self {
        Self::Positive
    }
}
impl From<bool> for NyarSign {
    fn from(value: bool) -> Self {
        match value {
            true => Self::Positive,
            false => Self::Negative,
        }
    }
}

impl From<Sign> for NyarSign {
    fn from(value: Sign) -> Self {
        match value {
            Sign::Minus => Self::Negative,
            Sign::NoSign => Self::Positive,
            Sign::Plus => Self::Positive,
        }
    }
}
