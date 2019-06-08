//! Compresses the input from stdin and writes the result to stdout.

extern crate primal;

pub mod auxiliary;

#[no_mangle]
/// - `n`: The nth prime
pub extern fn prime_n(n: usize) {}

#[no_mangle]
pub extern fn prime_n_range(min: usize, max: usize, step: usize) {}

#[no_mangle]
/// - `p`: A number
/// - `n`: default - 1
///   - `n < 0`: The next nth prime
///   - `n = 0`: Error
///   - `n > 0`: The previous nth prime
pub extern fn prime_next(p: usize, n: isize) {}

#[no_mangle]
pub extern fn prime_count(n: usize) {}

#[no_mangle]
pub extern fn prime_count_range(min: usize, max: usize, step: usize) {}