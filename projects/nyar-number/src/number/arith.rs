use super::*;

impl One for NyarNumber {
    fn one() -> Self {
        todo!()
    }
}

impl Zero for NyarNumber {
    fn zero() -> Self {
        todo!()
    }

    fn is_zero(&self) -> bool {
        todo!()
    }
}

impl Neg for NyarNumber {
    type Output = Self;

    fn neg(self) -> Self::Output {
        todo!()
    }
}
impl Add for NyarNumber {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        todo!()
    }
}
impl Sub for NyarNumber {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl Mul for NyarNumber {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        match self {
            NyarNumber::Rational(lhs) => match other {
                NyarNumber::Rational(rhs) => lhs.mul(rhs).into(),
                NyarNumber::Decimal(_) => {
                    todo!()
                }
                NyarNumber::Complex(_) => {
                    todo!()
                }
            },
            NyarNumber::Decimal(_) => {
                todo!()
            }
            NyarNumber::Complex(_) => {
                todo!()
            }
        }
    }
}

impl Div for NyarNumber {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        match self {
            NyarNumber::Rational(_) => match other {
                NyarNumber::Rational(_) => {
                    todo!()
                }
                NyarNumber::Decimal(_) => {
                    todo!()
                }
                NyarNumber::Complex(_) => {
                    todo!()
                }
            },
            NyarNumber::Decimal(_) => {
                todo!()
            }
            NyarNumber::Complex(_) => {
                todo!()
            }
        }
    }
}

impl Rem for NyarNumber {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

// impl NumOps for NyarNumber {}

impl Signed for NyarNumber {
    fn abs(&self) -> Self {
        todo!()
    }

    fn abs_sub(&self, other: &Self) -> Self {
        todo!()
    }

    fn signum(&self) -> Self {
        todo!()
    }

    fn is_positive(&self) -> bool {
        todo!()
    }

    fn is_negative(&self) -> bool {
        todo!()
    }
}
