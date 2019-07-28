use num::BigInt;

use prime::is_fermat_prime;

#[cfg(test)]
mod fermat_prime {
    use super::*;

    #[test]
    fn num_2() {
        let result = is_fermat_prime(2);
        assert_eq!(result, false);
    }

    #[test]
    fn num_5() {
        let result = is_fermat_prime(BigInt::from(5));
        assert_eq!(result, true);
    }
}
