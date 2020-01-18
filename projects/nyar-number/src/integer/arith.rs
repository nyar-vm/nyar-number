use super::*;
use num::{traits::NumOps, Num};
use std::ops::{Neg, Rem};

impl Zero for NyarUnsigned {
    fn zero() -> Self {
        Self { _repr: BigUint::default() }
    }

    fn is_zero(&self) -> bool {
        self._repr.is_zero()
    }
}

impl One for NyarUnsigned {
    fn one() -> Self {
        Self { _repr: BigUint::one() }
    }
}

impl PartialEq for NyarInteger {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

impl Zero for NyarInteger {
    fn zero() -> Self {
        todo!()
    }

    fn is_zero(&self) -> bool {
        todo!()
    }
}

impl Add for NyarInteger {
    type Output = ();

    fn add(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl One for NyarInteger {
    fn one() -> Self {
        todo!()
    }
}

impl Mul for NyarInteger {
    type Output = ();

    fn mul(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl NumOps for NyarInteger {}

impl Sub for NyarInteger {
    type Output = ();

    fn sub(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl Div for NyarInteger {
    type Output = ();

    fn div(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl Rem for NyarInteger {
    type Output = ();

    fn rem(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl Neg for NyarInteger {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self { sign: Default::default(), digits: Default::default() }
    }
}

impl Neg for NyarSign {
    type Output = ();

    fn neg(self) -> Self::Output {
        todo!()
    }
}

// impl Signed for NyarInteger {
//     fn abs(&self) -> Self {
//         todo!()
//     }
//
//     fn abs_sub(&self, other: &Self) -> Self {
//         todo!()
//     }
//
//     fn signum(&self) -> Self {
//         todo!()
//     }
//
//     fn is_positive(&self) -> bool {
//         todo!()
//     }
//
//     fn is_negative(&self) -> bool {
//         todo!()
//     }
// }

impl Add for NyarUnsigned {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self { _repr: self._repr.add(rhs._repr) }
    }
}
impl Sub for NyarUnsigned {
    type Output = Self;

    fn sub(self, _: Self) -> Self::Output {
        unreachable!()
    }
}

impl Mul for NyarUnsigned {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self { _repr: self._repr.mul(rhs._repr) }
    }
}
impl Div for NyarUnsigned {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        todo!()
    }
}
