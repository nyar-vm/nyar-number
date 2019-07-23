extern crate num_bigint;
extern crate primal;

pub mod auxiliary;
pub mod checker;
pub mod prime_count;
pub mod special;

pub use checker::is_fermat_prime;

/*
#[no_mangle]
/// - `n`: The nth prime
pub extern "C" fn prime_n(n: usize) {}

#[no_mangle]
pub extern "C" fn prime_n_range(min: usize, max: usize, step: usize) {}

#[no_mangle]
/// - `p`: A number
/// - `n`: default - 1
///   - `n < 0`: The next nth prime
///   - `n = 0`: Error
///   - `n > 0`: The previous nth prime
pub extern "C" fn prime_next(p: usize, n: isize) {}

#[no_mangle]
pub extern "C" fn prime_count(n: usize) {}

#[no_mangle]
pub extern "C" fn prime_count_range(min: usize, max: usize, step: usize) {}

#[no_mangle]
pub extern "C" fn prime_sum(n: usize) {}

#[no_mangle]
pub extern "C" fn prime_sum_range(min: usize, max: usize, step: usize) {}

*/
