use num::BigInt;
use super::truncatable_primes_left::LEFT_TRUNCATABLE_PRIMES;
use super::truncatable_primes_right::RIGHT_TRUNCATABLE_PRIMES;

pub fn get_left_truncatable_primes() -> Vec<BigInt> {
    let iter = LEFT_TRUNCATABLE_PRIMES.to_vec().into_iter();
    iter.map(|n| BigInt::from(n)).collect()
}

pub fn get_right_truncatable_primes() -> Vec<BigInt> {
    let iter = RIGHT_TRUNCATABLE_PRIMES.to_vec().into_iter();
    iter.map(|n| BigInt::from(n)).collect()
}

#[test]
fn count() {
    let lrps = get_left_truncatable_primes();
    let rrps = get_right_truncatable_primes();
    assert_eq!(lrps.len(), 4260);
    assert_eq!(rrps.len(), 83);
}
