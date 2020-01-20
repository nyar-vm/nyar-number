use super::*;
use crate::{One, Zero};
use num::{traits::NumOps, Num, Signed};
use std::ops::{Add, Div, Neg, Rem, Sub};

impl Num for NyarRational {
    type FromStrRadixErr = ();

    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        todo!()
    }
}

impl Zero for NyarRational {
    fn zero() -> Self {
        todo!()
    }

    fn is_zero(&self) -> bool {
        todo!()
    }
}

impl One for NyarRational {
    fn one() -> Self {
        todo!()
    }
}

impl Add for NyarRational {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl Mul for NyarRational {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let sign = self.sign.mul(rhs.sign);
        let numerator = Gc::new(self.numerator.get().clone().mul(rhs.numerator.get().clone()));
        let denominator = Gc::new(self.denominator.get().clone().mul(rhs.denominator.get().clone()));
        NyarRational { sign, numerator, denominator }
    }
}

impl Sub for NyarRational {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl Div for NyarRational {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl Rem for NyarRational {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl Neg for NyarRational {
    type Output = Self;

    fn neg(self) -> Self::Output {
        todo!()
    }
}

impl Signed for NyarRational {
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
