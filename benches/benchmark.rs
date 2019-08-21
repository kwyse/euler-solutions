use criterion::{black_box, criterion_group, criterion_main};
use criterion::{Criterion, ParameterizedBenchmark};

use euler_solutions::p001::sum_all_multiples;
use euler_solutions::p002::sum_of_even_value_fibs;
use euler_solutions::p003::largest_prime_factor;
use euler_solutions::p004::largest_palindrome;
use euler_solutions::p005::{lcm, product_of_minimal_prime_factors};
use euler_solutions::p006::difference;

fn benchmark(c: &mut Criterion) {
    c.bench_function("p001", |b| {
        b.iter(|| sum_all_multiples(black_box(&[3, 5]), black_box(1_000)))
    });
}

criterion_main!(benches);
criterion_group! {
    name = benches;
    config = Criterion::default().noise_threshold(0.1);
    targets = benchmark
}
