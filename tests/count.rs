use prime::special::truncatable_prime::{
    get_left_truncatable_primes, get_right_truncatable_primes,
};

#[cfg(test)]
mod truncatable_primes {
    use super::*;

    #[test]
    fn left() {
        let lrps = get_left_truncatable_primes();
        assert_eq!(lrps.len(), 4260);
    }

    #[test]
    fn right() {
        let rrps = get_right_truncatable_primes();
        assert_eq!(rrps.len(), 83);
    }
}
