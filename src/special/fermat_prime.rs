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
    pub fn is_fermat_prime() {
        // $F_n$ is a prime，only if $3^{{{\frac  {F_{n}-1}{2}}}}\equiv -1{\pmod  {F_{n}}}$
        unimplemented!()
    }
    pub fn value(&self) -> BigInt {
        let b = BigInt::from(2);
        let index = &self.index;
        let e = power(b.clone(), index.clone());
        power(b, e) + 1
    }
}

impl<T> From<T> for FermatNumber
where
    BigInt: From<T>,
{
    fn from(i: T) -> FermatNumber {
        let v = BigInt::from(i);
        //let f = log2(log2(v - 1));
        FermatNumber { index: v }
    }
}

fn known_fermat_indexes() -> Vec<BigInt> {
    const I: &[i8] = &[0, 1, 2, 3, 4];
    I.to_vec().iter().map(|x| BigInt::from(*x)).collect()
}

pub fn get_fermat_primes() -> Vec<BigInt> {
    let v = known_fermat_indexes().into_iter();
    v.map(|x| FermatNumber::from(x.clone()).value()).collect()
}
