use super::*;
use num::Integer;

impl Zero for NyarInteger {
    fn zero() -> Self {
        Self { sign: Sign::NoSign, digits: NyarDigits::zero() }
    }

    fn is_zero(&self) -> bool {
        self.digits.is_zero()
    }
}

impl One for NyarInteger {
    fn one() -> Self {
        Self { sign: Sign::Plus, digits: NyarDigits::one() }
    }
    fn is_one(&self) -> bool
    where
        Self: PartialEq,
    {
        self.sign == Sign::Plus && self.digits.is_one()
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
            _ => self.delegate().add(other.delegate()).into(),
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
            _ => self.delegate().sub(other.delegate()).into(),
        }
    }
}

impl Mul for NyarInteger {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let sign = self.sign.mul(rhs.sign);
        let lhs_view = self.digits.clone();
        let rhs_view = rhs.digits.clone();
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
            let value = self.digits.delegate().mul(rhs.digits.delegate());
            Self { sign, digits: NyarDigits::from(value) }
        }
    }
}

impl Div for NyarInteger {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        if rhs.is_one() {
            return self;
        }
        else if rhs.digits.is_one() {
            return Self { sign: -self.sign, digits: self.digits.clone() };
        }
        else {
            self.delegate().div(rhs.delegate()).into()
        }
    }
}

impl Rem for NyarInteger {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        self.delegate().rem(rhs.delegate()).into()
    }
}

// impl NumOps for NyarInteger {}

impl Signed for NyarInteger {
    fn abs(&self) -> Self {
        Self { sign: Sign::Plus, digits: self.digits.clone() }
    }

    fn abs_sub(&self, other: &Self) -> Self {
        self.delegate().abs_sub(&other.delegate()).into()
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
        self.delegate().div_floor(&other.delegate()).into()
    }

    fn mod_floor(&self, other: &Self) -> Self {
        self.delegate().mod_floor(&other.delegate()).into()
    }

    fn gcd(&self, other: &Self) -> Self {
        self.delegate().gcd(&other.delegate()).into()
    }

    fn lcm(&self, other: &Self) -> Self {
        self.delegate().lcm(&other.delegate()).into()
    }

    fn divides(&self, other: &Self) -> bool {
        self.digits.delegate().is_multiple_of(&other.digits.delegate())
    }

    fn is_multiple_of(&self, other: &Self) -> bool {
        self.digits.delegate().is_multiple_of(&other.digits.delegate())
    }

    fn is_even(&self) -> bool {
        self.digits.delegate().is_even()
    }

    fn is_odd(&self) -> bool {
        self.digits.delegate().is_odd()
    }

    fn div_rem(&self, other: &Self) -> (Self, Self) {
        let (a, b) = self.delegate().div_rem(&other.delegate());
        (a.into(), b.into())
    }
}
