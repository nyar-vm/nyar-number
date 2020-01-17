use std::ops::{Div, Rem};

use num::cast::ToPrimitive;
use num::{pow, BigInt};

pub fn div_rem<T: Div<Output = T> + Rem<Output = T> + Copy>(x: T, y: T) -> (T, T) {
    let quotient = x / y;
    let remainder = x % y;
    (quotient, remainder)
}

pub fn power(b: BigInt, e: BigInt) -> BigInt {
    pow(b, e.to_usize().unwrap())
}
