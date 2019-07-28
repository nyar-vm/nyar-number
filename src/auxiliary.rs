use std::ops::{Div, Rem};

pub fn div_rem<T: Div<Output = T> + Rem<Output = T> + Copy>(x: T, y: T) -> (T, T) {
    let quotient = x / y;
    let remainder = x % y;
    (quotient, remainder)
}
