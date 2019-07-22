use num::BigInt;
use super::truncatable_primes_left::LEFT_TRUNCATABLE_PRIMES;
use std::str::FromStr;

pub fn get_left_truncatable_primes() -> Vec<BigInt> {
    let iter = LEFT_TRUNCATABLE_PRIMES.to_vec().into_iter();
    iter.map(|n| BigInt::from(n)).collect()
}


fn get_right_truncatable_primes() -> Vec<BigInt> {
    const TEXT: &str = include_str!("truncatable_primes_right.txt");
    let mut result: Vec<BigInt> = vec![];
    for s in TEXT.lines() {
        let i = BigInt::from_str(s);
        result.push(i.unwrap())
    };
    result
}

#[test]
fn count() {
    let lrps = get_left_truncatable_primes();
    let rrps = get_right_truncatable_primes();
    assert_eq!(lrps.len(), 4260);
    assert_eq!(rrps.len(), 83);
}


#[test]
fn print() {
    assert_eq!(get_right_truncatable_primes()[0], BigInt::from(2));
}