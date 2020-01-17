use num::BigInt;
use std::str::FromStr;

pub fn get_left_truncatable_primes() -> Vec<BigInt> {
    const TEXT: &str = include_str!("truncatable_primes_left.txt");
    let mut result: Vec<BigInt> = vec![];
    for s in TEXT.lines() {
        let i = BigInt::from_str(s);
        result.push(i.unwrap())
    }
    result
}

pub fn get_right_truncatable_primes() -> Vec<BigInt> {
    const TEXT: &str = include_str!("truncatable_primes_right.txt");
    let mut result: Vec<BigInt> = vec![];
    for s in TEXT.lines() {
        let i = BigInt::from_str(s);
        result.push(i.unwrap())
    }
    result
}
