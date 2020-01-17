use criterion::Criterion;
use primal::Sieve;

fn warmup_100w() {
    // Prime@1000000
    Sieve::new(15485863);
}

fn warmup_1000w() {
    // Prime@10000000
    Sieve::new(179424673);
}

fn warmup_10000w() {
    // Prime@100000000
    Sieve::new(2038074743);
}

pub fn warmup_benches(c: &mut Criterion) {
    c.bench_function("Warmup: 1000000", |x| x.iter(|| warmup_100w()));
    c.bench_function("Warmup: 10000000", |x| x.iter(|| warmup_1000w()));
    c.bench_function("Warmup: 100000000", |b| b.iter(|| warmup_10000w()));
}
