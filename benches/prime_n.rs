use criterion::{Criterion, Fun};
use primal::Sieve;

pub fn prime_n_benches(c: &mut Criterion) {
    let sieve = Sieve::new(15485863);
    let prime_1w = Fun::new("[1:10000:100]", move |b, _| {
        b.iter(|| {
            let range = (1..10000).step_by(100);
            let primes = range.map(|n| sieve.nth_prime(n));
            primes.fold(0, |a, b| a + b)
        })
    });
    let sieve = Sieve::new(15485863);
    let prime_10w = Fun::new("[1:100000:1000]", move |b, _| {
        b.iter(|| {
            let range = (1..100000).step_by(1000);
            let primes = range.map(|n| sieve.nth_prime(n));
            primes.fold(0, |a, b| a + b)
        })
    });
    let sieve = Sieve::new(15485863);
    let prime_100w = Fun::new("[1:1000000:10000]", move |b, _| {
        b.iter(|| {
            let range = (1..1000000).step_by(10000);
            let primes = range.map(|n| sieve.nth_prime(n));
            primes.fold(0, |a, b| a + b)
        })
    });
    let tests = vec![prime_1w, prime_10w, prime_100w];
    c.bench_functions("PrimeN", tests, ());
}

/*
fn prime_n(sieve: &Sieve, n: usize) -> usize {
    sieve.nth_prime(n)
}

pub fn prime_n_benches(c: &mut Criterion) {
    let ctx = Sieve::new(15485863);
    c.bench_function("Prime 10000", |b| b.iter(|| { prime_n(&ctx, 10000) }));
    c.bench_function("Prime 100000", |b| b.iter(|| { prime_n(&ctx, 100000) }));
}
*/
