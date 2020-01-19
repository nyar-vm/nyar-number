use super::*;
use std::cmp::Ordering;

impl Zero for NyarInteger {
    fn zero() -> Self {
        Self { sign: Sign::NoSign, digits: Default::default() }
    }

    fn is_zero(&self) -> bool {
        self.digits.get().is_zero()
    }
}

impl One for NyarInteger {
    fn one() -> Self {
        Self { sign: Sign::NoSign, digits: Gc::new(NyarUnsigned::one()) }
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
            (_, Sign::NoSign) => self,
            (Sign::NoSign, _) => other,
            (Sign::Plus, Sign::Plus) | (Sign::Minus, Sign::Minus) => {
                let value = self.digits.get().clone() + other.digits.get().clone();
                Self { sign: self.sign, digits: Gc::new(value) }
            }
            (Sign::Plus, Sign::Minus) | (Sign::Minus, Sign::Plus) => match self.digits.cmp(&other.digits) {
                Ordering::Less => {
                    let value = other.digits.get().clone() - self.digits.get().clone();
                    Self { sign: other.sign, digits: Gc::new(value) }
                }
                Ordering::Greater => {
                    let value = self.digits.get().clone() - other.digits.get().clone();
                    Self { sign: self.sign, digits: Gc::new(value) }
                }
                Ordering::Equal => Zero::zero(),
            },
        }
    }
}
impl Sub for NyarInteger {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        match (self.sign, other.sign) {
            (_, Sign::NoSign) => self,
            (Sign::NoSign, _) => -other,
            (Sign::Plus, Sign::Minus) | (Sign::Minus, Sign::Plus) => {
                let value = self.digits.get().clone() + other.digits.get().clone();
                Self { sign: self.sign, digits: Gc::new(value) }
            }
            (Sign::Plus, Sign::Plus) | (Sign::Minus, Sign::Minus) => match self.digits.cmp(&other.digits) {
                Ordering::Less => {
                    let value = other.digits.get().clone() - self.digits.get().clone();
                    Self { sign: -self.sign, digits: Gc::new(value) }
                }
                Ordering::Greater => {
                    let value = self.digits.get().clone() + other.digits.get().clone();
                    Self { sign: self.sign, digits: Gc::new(value) }
                }
                Ordering::Equal => Zero::zero(),
            },
        }
    }
}

impl Mul for NyarInteger {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let sign = self.sign.mul(rhs.sign);
        let value = self.digits.get().clone()._repr.mul(rhs.digits.get().clone()._repr);
        Self { sign, digits: Gc::new(NyarUnsigned { _repr: value }) }
    }
}

impl Div for NyarInteger {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl Rem for NyarInteger {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

// impl NumOps for NyarInteger {}

impl Signed for NyarInteger {
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
