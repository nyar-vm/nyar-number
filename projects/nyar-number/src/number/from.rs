use super::*;
use num::Num;

impl Num for NyarNumber {
    type FromStrRadixErr = ();

    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        todo!()
    }
}

impl From<NyarInteger> for NyarNumber {
    fn from(value: NyarInteger) -> Self {
        Self::Integer(value)
    }
}
