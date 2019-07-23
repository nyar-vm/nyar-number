//! http://mathworld.wolfram.com/FermatNumber.html

use std::str::FromStr;

use num_bigint::BigInt;

/// A Fermat prime is the Fermat number $F_n=2^{2^n}+1$ that is prime.
pub struct FermatPrime {
    ordinal: BigInt,
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

impl FermatPrime {
    pub fn new() -> FermatPrime {
        FermatPrime {
            ordinal: BigInt::from(0),
        }
    }
    //pub fn nth(n: isize) -> BigInt {}
}

impl From<i64> for FermatPrime {
    fn from(i: i64) -> FermatPrime {
        let v = BigInt::from(i);
        FermatPrime { ordinal: v }
    }
}
