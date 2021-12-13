use std::str::FromStr;

use num::BigInt;

pub struct MersenneNumber {
    index: BigInt,
}

impl MersenneNumber {
    pub fn new<T>(i: T) -> MersenneNumber
    where
        BigInt: From<T>,
    {
        let int = BigInt::from(i);
        MersenneNumber { index: int }
    }
    pub fn unwrap(&self) -> &BigInt {
        &self.index
    }
    //pub fn nth(n: isize) -> BigInt {}
}

pub fn get_mersenne_primes() -> Vec<BigInt> {
    // notice this is index
    const TEXT: &str = include_str!("mersenne_primes.txt");
    let mut result: Vec<BigInt> = vec![];
    for s in TEXT.lines() {
        let i = BigInt::from_str(s).unwrap();
        result.push(i)
    }
    result
}
