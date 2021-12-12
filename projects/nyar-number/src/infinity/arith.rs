use super::*;
use crate::{Num, One, ToPrimitive, Zero};
use bigdecimal::num_traits::{NumCast, NumOps};
use num::Float;
use std::{
    cmp::Ordering,
    ops::{Div, Mul, Neg, Rem, Sub},
};
impl Neg for NyarInfinity {
    type Output = Self;

    fn neg(self) -> Self::Output {
        todo!()
    }
}
impl Add for NyarInfinity {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match (self.sign, rhs.sign) {
            (Minus, Minus) => Self { sign: Minus },
            (Plus, Plus) => Self { sign: Plus },
            _ => Self { sign: NoSign },
        }
    }
}
impl Sub for NyarInfinity {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match (self.sign, rhs.sign) {
            (Minus, Minus) => Self { sign: Minus },
            (Plus, Plus) => Self { sign: Plus },
            _ => Self { sign: NoSign },
        }
    }
}

impl Num for NyarInfinity {
    type FromStrRadixErr = ();

    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        todo!()
    }
}

impl PartialEq for NyarInfinity {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

impl Zero for NyarInfinity {
    fn zero() -> Self {
        todo!()
    }

    fn is_zero(&self) -> bool {
        todo!()
    }
}

impl Add<Self, Output = Self> for NyarInfinity {
    type Output = ();

    fn add(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl One for NyarInfinity {
    fn one() -> Self {
        todo!()
    }
}

impl Mul<Self, Output = Self> for NyarInfinity {
    type Output = ();

    fn mul(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl NumOps for NyarInfinity {}

impl Sub<Self, Output = Self> for NyarInfinity {
    type Output = ();

    fn sub(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl Div<Self, Output = Self> for NyarInfinity {
    type Output = ();

    fn div(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl Rem<Self, Output = Self> for NyarInfinity {
    type Output = ();

    fn rem(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl Copy for NyarInfinity {}

impl NumCast for NyarInfinity {
    fn from<T: ToPrimitive>(n: T) -> Option<Self> {
        todo!()
    }
}

impl ToPrimitive for NyarInfinity {
    fn to_i64(&self) -> Option<i64> {
        todo!()
    }

    fn to_u64(&self) -> Option<u64> {
        todo!()
    }
}

impl PartialOrd for NyarInfinity {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        todo!()
    }
}
