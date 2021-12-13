use super::*;
use crate::NyarReal::Infinity;
use NyarReal::{Decimal, Rational};
use Sign::{Minus, NoSign, Plus};

impl One for NyarReal {
    fn one() -> Self {
        Rational(NyarRational::one())
    }
}

impl Zero for NyarReal {
    fn zero() -> Self {
        Rational(NyarRational::zero())
    }

    fn is_zero(&self) -> bool {
        match self {
            Rational(v) => v.is_zero(),
            Decimal(v) => v.is_zero(),
            Infinity { .. } => false,
        }
    }
}

impl Neg for NyarReal {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Infinity(v) => Infinity(v.neg()),
            Rational(v) => Rational(v.neg()),
            Decimal(v) => Decimal(v.neg()),
        }
    }
}

impl Add for NyarReal {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        match (self, other) {
            (Infinity(lhs), Infinity(rhs)) => Infinity(lhs.add(rhs)),
            (Infinity(i), _) | (_, Infinity(i)) => Infinity(i),
            (Decimal(lhs), Decimal(rhs)) => Decimal(lhs.add(rhs)),
            (Decimal(lhs), Rational(rhs)) => Decimal(lhs.add(rhs.as_decimal())),
            (Rational(lhs), Decimal(rhs)) => Decimal(lhs.as_decimal().add(rhs)),
            (Rational(lhs), Rational(rhs)) => Rational(lhs.add(rhs)),
        }
    }
}
impl Sub for NyarReal {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        match (self, other) {
            (Infinity(lhs), Infinity(rhs)) => Infinity(lhs.sub(rhs)),
            (Infinity(i), _) => Infinity(i),
            (_, Infinity(i)) => Infinity(i.neg()),
            (Decimal(lhs), Decimal(rhs)) => Decimal(lhs.sub(rhs)),
            (Decimal(lhs), Rational(rhs)) => Decimal(lhs.sub(rhs.as_decimal())),
            (Rational(lhs), Decimal(rhs)) => Decimal(lhs.as_decimal().sub(rhs)),
            (Rational(lhs), Rational(rhs)) => Rational(lhs.sub(rhs)),
        }
    }
}

impl Mul for NyarReal {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        match (self, other) {
            (Infinity(lhs), Infinity(rhs)) => Infinity(lhs.mul(rhs)),
            (Infinity(i), _) | (_, Infinity(i)) => Infinity(i),
            (Decimal(lhs), Decimal(rhs)) => Decimal(lhs.mul(rhs)),
            (Decimal(lhs), Rational(rhs)) => Decimal(lhs.mul(rhs.as_decimal())),
            (Rational(lhs), Decimal(rhs)) => Decimal(lhs.as_decimal().mul(rhs)),
            (Rational(lhs), Rational(rhs)) => Rational(lhs.mul(rhs)),
        }
    }
}

impl Div for NyarReal {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        match (self, other) {
            (_, Infinity(_)) => Infinity(NyarInfinity::INDETERMINATE),
            (Infinity(i), _) => Infinity(i),
            (Decimal(lhs), Decimal(rhs)) => lhs.safe_div(rhs),
            (Decimal(lhs), Rational(rhs)) => lhs.safe_div(rhs.as_decimal()),
            (Rational(lhs), Decimal(rhs)) => lhs.as_decimal().safe_div(rhs),
            (Rational(lhs), Rational(rhs)) => lhs.safe_div(rhs),
        }
    }
}

impl Rem for NyarReal {
    type Output = Self;

    fn rem(self, other: Self) -> Self::Output {
        match (self, other) {
            (_, Infinity(_)) => Infinity(NyarInfinity::INDETERMINATE),
            (Infinity(i), _) => Infinity(i),
            (Decimal(lhs), Decimal(rhs)) => lhs.safe_rem(rhs),
            (Decimal(lhs), Rational(rhs)) => lhs.safe_rem(rhs.as_decimal()),
            (Rational(lhs), Decimal(rhs)) => lhs.as_decimal().safe_rem(rhs),
            (Rational(lhs), Rational(rhs)) => lhs.safe_rem(rhs),
        }
    }
}

// impl NumOps for NyarNumber {}

impl Signed for NyarReal {
    fn abs(&self) -> Self {
        todo!()
    }

    fn abs_sub(&self, other: &Self) -> Self {
        match (self, other) {
            (Infinity(_), _) | (_, Infinity(_)) => Infinity(NyarInfinity::INDETERMINATE),
            (Decimal(lhs), Decimal(rhs)) => Decimal(lhs.abs_sub(rhs)),
            (Decimal(lhs), Rational(rhs)) => Decimal(lhs.abs_sub(&rhs.as_decimal())),
            (Rational(lhs), Decimal(rhs)) => Decimal(lhs.as_decimal().abs_sub(rhs)),
            (Rational(lhs), Rational(rhs)) => Rational(lhs.abs_sub(rhs)),
        }
    }

    fn signum(&self) -> Self {
        match self {
            Rational(v) => Rational(v.signum()),
            Decimal(v) => Decimal(v.signum()),
            Infinity(v) => Infinity(*v),
        }
    }

    fn is_positive(&self) -> bool {
        match self {
            Rational(v) => v.is_positive(),
            Decimal(v) => v.is_positive(),
            Infinity(v) => v.is_positive(),
        }
    }

    fn is_negative(&self) -> bool {
        match self {
            Rational(v) => v.is_negative(),
            Decimal(v) => v.is_negative(),
            Infinity(v) => v.is_negative(),
        }
    }
}
