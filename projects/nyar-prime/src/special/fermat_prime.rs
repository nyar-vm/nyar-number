//! http://mathworld.wolfram.com/FermatNumber.html

use std::str::FromStr;

use num::BigInt;

use crate::auxiliary::power;
use core::borrow::Borrow;

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
    pub fn is_fermat_prime(&self) {
        let index = &self.index;
        // $F_n$ is a prime，only if $3^{{{\frac  {F_{n}-1}{2}}}}\equiv -1{\pmod  {F_{n}}}$
        unimplemented!()
    }
    pub fn value(&self) -> BigInt {
        let index = &self.index;
        let b = BigInt::from(2);
        let e = power(b.clone(), index.clone());
        power(b, e) + 1
    }
}

impl From<BigInt> for FermatNumber {
    fn from(i: BigInt) -> FermatNumber {
        let v = i;
        //let f = log2(log2(v - 1));
        FermatNumber { index: v }
    }
}
impl From<i64> for FermatNumber {
    fn from(i: i64) -> FermatNumber {
        let v = BigInt::from(i);
        //let f = log2(log2(v - 1));
        FermatNumber { index: v }
    }
}
impl From<&str> for FermatNumber {
    fn from(i: &str) -> FermatNumber {
        let v = BigInt::parse_bytes(i.as_bytes(), 10).unwrap();
        //let f = log2(log2(v - 1));
        FermatNumber { index: v }
    }
}

const KNOWN_FERMAT_PRIME_INDEXES: &[i8] = &[0, 1, 2, 3, 4];
pub fn get_fermat_primes() -> Vec<BigInt> {
    let v = KNOWN_FERMAT_PRIME_INDEXES.to_vec();
    v.iter()
        .map(|x| BigInt::from(*x))
        .map(|x| FermatNumber::from(x).value())
        .collect()
}
