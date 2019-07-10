#[macro_use]
extern crate criterion;
use criterion::Criterion;
use std::time::Duration;

pub mod warmup;
use warmup::warmup_benches;
criterion_group! {
    name = warmup;
    config = Criterion::default()
        .sample_size(10)
        .warm_up_time(Duration::from_millis(1000));
    targets = warmup_benches
}

pub mod prime_n;
use prime_n::prime_n_benches;
criterion_group! {
    name = prime_n;
    config = Criterion::default()
        .sample_size(10)
        .warm_up_time(Duration::from_millis(1000));
    targets = prime_n_benches
}

criterion_main!(prime_n);
