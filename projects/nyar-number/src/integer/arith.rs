use super::*;
use crate::unsigned::{ONE, ZERO};
use num::Integer;

impl Zero for NyarInteger {
    fn zero() -> Self {
        Self { sign: Sign::NoSign, digits: ZERO.clone() }
    }

    fn is_zero(&self) -> bool {
        self.digits.get().is_zero()
    }
}

impl One for NyarInteger {
    fn one() -> Self {
        Self { sign: Sign::Plus, digits: ONE.clone() }
    }
}

impl Neg for NyarInteger {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self { sign: self.sign.neg(), digits: self.digits }
    }
}

impl Add for NyarInteger {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        match (self.sign, other.sign) {
            // non allocate path
            (_, Sign::NoSign) => self,
            (Sign::NoSign, _) => other,
            // non reusable path
            _ => self.wrapped().add(other.wrapped()).into(),
        }
    }
}
impl Sub for NyarInteger {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        match (self.sign, other.sign) {
            // non allocate path
            (_, Sign::NoSign) => self,
            (Sign::NoSign, _) => -other,
            // non reusable path
            _ => self.wrapped().sub(other.wrapped()).into(),
        }
    }
}

impl Mul for NyarInteger {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let sign = self.sign.mul(rhs.sign);
        let lhs_view = self.digits.get();
        let rhs_view = rhs.digits.get();
        if lhs_view.is_zero() || rhs_view.is_zero() {
            // non allocate path
            return NyarInteger::zero();
        }
        else if lhs_view.is_one() {
            // non allocate path
            return Self { sign, digits: rhs.digits.clone() };
        }
        else if rhs_view.is_one() {
            // non allocate path
            return Self { sign, digits: self.digits.clone() };
        }
        else {
            let value = self.digits.get().delegate().mul(rhs.digits.get().delegate());
            Self { sign, digits: Gc::new(NyarUnsigned::from(value)) }
        }
    }
}

impl Div for NyarInteger {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        if rhs.is_one() {
            return self;
        }
        else if rhs.digits.get().is_one() {
            return Self { sign: -self.sign, digits: self.digits.clone() };
        }
        else {
            self.wrapped().div(rhs.wrapped()).into()
        }
    }
}

impl Rem for NyarInteger {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        self.wrapped().rem(rhs.wrapped()).into()
    }
}

// impl NumOps for NyarInteger {}

impl Signed for NyarInteger {
    fn abs(&self) -> Self {
        Self { sign: Sign::Plus, digits: self.digits.clone() }
    }

    fn abs_sub(&self, other: &Self) -> Self {
        self.wrapped().abs_sub(&other.wrapped()).into()
    }

    fn signum(&self) -> Self {
        match self.sign {
            Sign::Minus => -Self::one(),
            Sign::NoSign => Self::zero(),
            Sign::Plus => Self::one(),
        }
    }

    fn is_positive(&self) -> bool {
        match self.sign {
            Sign::Minus => true,
            Sign::NoSign => false,
            Sign::Plus => false,
        }
    }

    fn is_negative(&self) -> bool {
        !self.is_positive()
    }
}

impl Integer for NyarInteger {
    fn div_floor(&self, other: &Self) -> Self {
        self.wrapped().div_floor(&other.wrapped()).into()
    }

    fn mod_floor(&self, other: &Self) -> Self {
        self.wrapped().mod_floor(&other.wrapped()).into()
    }

    fn gcd(&self, other: &Self) -> Self {
        self.wrapped().gcd(&other.wrapped()).into()
    }

    fn lcm(&self, other: &Self) -> Self {
        self.wrapped().lcm(&other.wrapped()).into()
    }

    fn divides(&self, other: &Self) -> bool {
        self.digits.get().delegate().is_multiple_of(&other.digits.get().delegate())
    }

    fn is_multiple_of(&self, other: &Self) -> bool {
        self.digits.get().delegate().is_multiple_of(&other.digits.get().delegate())
    }

    fn is_even(&self) -> bool {
        self.digits.get().delegate().is_even()
    }

    fn is_odd(&self) -> bool {
        self.digits.get().delegate().is_odd()
    }

    fn div_rem(&self, other: &Self) -> (Self, Self) {
        let (a, b) = self.wrapped().div_rem(&other.wrapped());
        (a.into(), b.into())
    }
}
