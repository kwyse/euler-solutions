use criterion::{black_box, criterion_group, criterion_main, Criterion};

use euler_solutions::p001::sum_of_multiples;
use euler_solutions::p002::sum_of_even_value_fibs;
use euler_solutions::p003::largest_prime_factor;
use euler_solutions::p004::largest_palindrome;
use euler_solutions::p005::smallest_multiple;

fn benchmark(c: &mut Criterion) {
    c.bench_function("p001", |b| {
        b.iter(|| sum_of_multiples(black_box(&[3, 5]), black_box(1_000)))
    });
    c.bench_function("p002", |b| {
        b.iter(|| sum_of_even_value_fibs(black_box(4_000_000)))
    });
    c.bench_function("p003", |b| {
        b.iter(|| largest_prime_factor(black_box(600_851_475_143)))
    });
    c.bench_function("p004", |b| {
        b.iter(|| largest_palindrome(black_box(1), black_box(1000)))
    });
    c.bench_function("p005", |b| {
        b.iter(|| smallest_multiple(black_box(1), black_box(20)))
    });
}

criterion_main!(benches);
criterion_group! {
    name = benches;
    config = Criterion::default().noise_threshold(0.1);
    targets = benchmark
}
