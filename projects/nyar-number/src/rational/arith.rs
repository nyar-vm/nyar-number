use super::*;
use crate::unsigned::ZERO;

impl Zero for NyarRational {
    fn zero() -> Self {
        Self { sign: Sign::Plus, numerator: ZERO.clone(), denominator: ONE.clone() }
    }

    fn is_zero(&self) -> bool {
        self.numerator.get().is_zero()
    }
}

impl One for NyarRational {
    fn one() -> Self {
        Self { sign: Sign::Plus, numerator: ONE.clone(), denominator: ONE.clone() }
    }
}
impl Neg for NyarRational {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self { sign: -self.sign, numerator: self.numerator, denominator: self.denominator }
    }
}
impl Add for NyarRational {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.delegate().add(rhs.delegate()).into()
    }
}
impl Sub for NyarRational {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.delegate().add(rhs.delegate()).into()
    }
}

impl Mul for NyarRational {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        // let sign = self.sign.mul(rhs.sign);
        // let numerator = Gc::new(self.numerator.get().clone().mul(rhs.numerator.get().clone()));
        // let denominator = Gc::new(self.denominator.get().clone().mul(rhs.denominator.get().clone()));
        // NyarRational { sign, numerator, denominator }
        self.delegate().mul(rhs.delegate()).into()
    }
}

impl Div for NyarRational {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        if rhs.is_zero() {
            return NyarRational::infinite(true).get().clone();
        }
        self.delegate().div(rhs.delegate()).into()
    }
}

impl Rem for NyarRational {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        self.delegate().rem(rhs.delegate()).into()
    }
}

impl Signed for NyarRational {
    fn abs(&self) -> Self {
        Self { sign: Sign::Plus, numerator: self.numerator.clone(), denominator: self.denominator.clone() }
    }

    fn abs_sub(&self, other: &Self) -> Self {
        self.delegate().abs_sub(&other.delegate()).into()
    }

    fn signum(&self) -> Self {
        self.delegate().signum().into()
    }

    fn is_positive(&self) -> bool {
        todo!()
    }

    fn is_negative(&self) -> bool {
        todo!()
    }
}
