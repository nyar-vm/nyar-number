use num::BigInt;

use crate::special::get_fermat_primes;

pub fn is_fermat_prime<T>(i: T) -> bool
where
    BigInt: From<T>,
{
    //
    let int = BigInt::from(i);
    let list = get_fermat_primes();
    list.contains(&int)
}
