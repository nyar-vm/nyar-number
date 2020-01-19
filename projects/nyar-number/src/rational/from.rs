use super::*;

impl From<BigRational> for NyarRational {
    fn from(value: BigRational) -> Self {
        let (num, den) = value.into();
        let sign = num.sign().mul(den.sign());
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
