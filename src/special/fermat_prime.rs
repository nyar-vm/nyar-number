//! http://mathworld.wolfram.com/FermatNumber.html

use std::str::FromStr;

use num::BigInt;

/// A Fermat prime is the Fermat number $F_n=2^{2^n}+1$ that is prime.
pub struct FermatNumber {
    index: BigInt,
}

impl FermatNumber {
    pub fn new<T>(i: T) -> FermatNumber
    where
        BigInt: From<T>,
    {
        let int = BigInt::from(i);
        FermatNumber { index: int }
    }
    //pub fn nth(n: isize) -> BigInt {}
    pub fn is_fermat_prime() {
        // $F_n$ is a prime，only if $3^{{{\frac  {F_{n}-1}{2}}}}\equiv -1{\pmod  {F_{n}}}$
        unimplemented!()
    }
}

impl<T> From<T> for FermatNumber {
    fn from(i: T) -> FermatNumber
    where
        BigInt: From<T>,
    {
        let v = BigInt::from(i);
        //let f = log2(log2(v - 1));
        FermatNumber { index: v }
    }
}

pub fn get_fermat_primes() -> Vec<BigInt> {
    const TEXT: &str = include_str!("fermat_primes.txt");
    let mut result: Vec<BigInt> = vec![];
    for s in TEXT.lines() {
        let i = BigInt::from_str(s).unwrap();
        result.push(i)
    }
    result
}
