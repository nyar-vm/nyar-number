use super::*;
use num::Signed;

impl From<BigRational> for NyarRational {
    fn from(value: BigRational) -> Self {
        let sign = NyarSign::from(value.is_positive());
        let (num, den) = value.into();
        let num = NyarInteger::from(num);
        let den = NyarInteger::from(den);
        NyarRational { sign, numerator: num.digits, denominator: den.digits }
    }
}

impl FromStr for NyarRational {
    type Err = IntErrorKind;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match BigRational::from_str(s) {
            Ok(o) => Ok(NyarRational::from(o)),
            Err(_) => Err(IntErrorKind::InvalidDigit),
        }
    }
}
