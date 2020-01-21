use super::*;

impl One for NyarNumber {
    fn one() -> Self {
        Self::Rational(NyarRational::one())
    }
}

impl Zero for NyarNumber {
    fn zero() -> Self {
        Self::Rational(NyarRational::zero())
    }

    fn is_zero(&self) -> bool {
        match self {
            Self::Rational(v) => v.is_zero(),
            Self::Decimal(v) => v.is_zero(),
            Self::Complex(v) => v.is_zero(),
        }
    }
}

impl Neg for NyarNumber {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Self::Rational(v) => Self::Rational(v.neg()),
            Self::Decimal(v) => Self::Decimal(v.neg()),
            Self::Complex(v) => Self::Complex(v.neg()),
        }
    }
}

impl Add for NyarNumber {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        match self {
            Self::Rational(lhs) => match other {
                Self::Rational(rhs) => Self::Rational(lhs.add(rhs)),
                Self::Decimal(rhs) => Self::Decimal(NyarDecimal::from(lhs).add(rhs)),
                Self::Complex(_) => {
                    todo!()
                }
            },
            Self::Decimal(lhs) => match other {
                Self::Rational(rhs) => Self::Decimal(lhs.add(NyarDecimal::from(rhs))),
                Self::Decimal(rhs) => Self::Decimal(lhs.add(rhs)),
                Self::Complex(_) => {
                    todo!()
                }
            },
            Self::Complex(_) => {
                todo!()
            }
        }
    }
}
impl Sub for NyarNumber {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        match self {
            Self::Rational(lhs) => match rhs {
                Self::Rational(rhs) => Self::Rational(lhs.sub(rhs)),
                Self::Decimal(rhs) => Self::Decimal(NyarDecimal::from(lhs).sub(rhs)),
                Self::Complex(_) => {
                    todo!()
                }
            },
            Self::Decimal(lhs) => match rhs {
                Self::Rational(rhs) => Self::Decimal(lhs.sub(NyarDecimal::from(rhs))),
                Self::Decimal(rhs) => Self::Decimal(lhs.sub(rhs)),
                Self::Complex(_) => {
                    todo!()
                }
            },
            Self::Complex(_) => {
                todo!()
            }
        }
    }
}

impl Mul for NyarNumber {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        match self {
            Self::Rational(lhs) => match rhs {
                Self::Rational(rhs) => Self::Rational(lhs.mul(rhs)),
                Self::Decimal(rhs) => Self::Decimal(NyarDecimal::from(lhs).mul(rhs)),
                Self::Complex(_) => {
                    todo!()
                }
            },
            Self::Decimal(lhs) => match rhs {
                Self::Rational(rhs) => Self::Decimal(lhs.mul(NyarDecimal::from(rhs))),
                Self::Decimal(rhs) => Self::Decimal(lhs.mul(rhs)),
                Self::Complex(_) => {
                    todo!()
                }
            },
            Self::Complex(_) => {
                todo!()
            }
        }
    }
}

impl Div for NyarNumber {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        match self {
            Self::Rational(lhs) => match other {
                Self::Rational(rhs) => Self::Rational(lhs.div(rhs)),
                Self::Decimal(rhs) => Self::Decimal(NyarDecimal::from(lhs).div(rhs)),
                Self::Complex(_) => {
                    todo!()
                }
            },
            Self::Decimal(lhs) => match other {
                Self::Rational(rhs) => Self::Decimal(lhs.div(NyarDecimal::from(rhs))),
                Self::Decimal(rhs) => Self::Decimal(lhs.div(rhs)),
                Self::Complex(_) => {
                    todo!()
                }
            },
            Self::Complex(_) => {
                todo!()
            }
        }
    }
}

impl Rem for NyarNumber {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        match self {
            Self::Rational(lhs) => match rhs {
                Self::Rational(rhs) => Self::Rational(lhs.rem(rhs)),
                Self::Decimal(rhs) => Self::Decimal(NyarDecimal::from(lhs).rem(rhs)),
                Self::Complex(_) => {
                    todo!()
                }
            },
            Self::Decimal(lhs) => match rhs {
                Self::Rational(rhs) => Self::Decimal(lhs.rem(NyarDecimal::from(rhs))),
                Self::Decimal(rhs) => Self::Decimal(lhs.rem(rhs)),
                Self::Complex(_) => {
                    todo!()
                }
            },
            Self::Complex(_) => {
                todo!()
            }
        }
    }
}

// impl NumOps for NyarNumber {}

impl Signed for NyarNumber {
    fn abs(&self) -> Self {
        todo!()
    }

    fn abs_sub(&self, rhs: &Self) -> Self {
        match self {
            Self::Rational(lhs) => match rhs {
                Self::Rational(rhs) => Self::Rational(lhs.abs_sub(rhs)),
                Self::Decimal(rhs) => Self::Decimal(NyarDecimal::from(lhs.clone()).abs_sub(rhs)),
                Self::Complex(_) => {
                    todo!()
                }
            },
            Self::Decimal(lhs) => match rhs {
                Self::Rational(rhs) => Self::Decimal(lhs.abs_sub(&NyarDecimal::from(rhs.clone()))),
                Self::Decimal(rhs) => Self::Decimal(lhs.abs_sub(rhs)),
                Self::Complex(_) => {
                    todo!()
                }
            },
            Self::Complex(_) => {
                todo!()
            }
        }
    }

    fn signum(&self) -> Self {
        match self {
            Self::Rational(v) => Self::Rational(v.signum()),
            Self::Decimal(v) => Self::Decimal(v.signum()),
            Self::Complex(v) => Self::Complex(v.signum()),
        }
    }

    fn is_positive(&self) -> bool {
        match self {
            Self::Rational(v) => v.is_positive(),
            Self::Decimal(v) => v.is_positive(),
            Self::Complex(v) => v.is_positive(),
        }
    }

    fn is_negative(&self) -> bool {
        match self {
            Self::Rational(v) => v.is_negative(),
            Self::Decimal(v) => v.is_negative(),
            Self::Complex(v) => v.is_negative(),
        }
    }
}
