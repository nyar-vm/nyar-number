pub mod primorial;
pub mod primorial_pi;

pub mod fermat_prime;
pub mod mersenne_prime;

pub mod truncatable_prime;

pub use fermat_prime::{get_fermat_primes, FermatNumber};
pub use truncatable_prime::{get_left_truncatable_primes, get_right_truncatable_primes};
