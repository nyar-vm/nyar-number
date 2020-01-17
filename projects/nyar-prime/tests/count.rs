use prime::{get_fermat_primes, get_left_truncatable_primes, get_right_truncatable_primes};

#[cfg(test)]
mod fermat_primes {
    use super::*;

    #[test]
    fn count() {
        let f = get_fermat_primes();
        assert_eq!(f.len(), 5);
    }
}

#[cfg(test)]
mod truncatable_primes {
    use super::*;

    #[test]
    fn count_left() {
        let ltps = get_left_truncatable_primes();
        assert_eq!(ltps.len(), 4260);
    }

    #[test]
    fn count_right() {
        let rtps = get_right_truncatable_primes();
        assert_eq!(rtps.len(), 83);
    }
}
