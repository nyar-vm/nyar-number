use super::*;
use std::ops::{Div, Sub};

impl Zero for NyarInteger {
    fn zero() -> Self {
        Self { _repr: UBig::default() }
    }

    fn is_zero(&self) -> bool {
        self._repr.is_zero()
    }
}

impl One for NyarInteger {
    fn one() -> Self {
        Self { _repr: UBig::ONE }
    }
}

impl Add for NyarInteger {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self { _repr: self._repr.add(rhs._repr) }
    }
}
// impl Sub for NyarInteger {
//     type Output = Self;
//
//     fn sub(self, _: Self) -> Self::Output {
//         unreachable!()
//     }
// }

impl Mul for NyarInteger {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self { _repr: self._repr.mul(rhs._repr) }
    }
}
impl Div for NyarInteger {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        todo!()
    }
}
