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
    c.bench_function("p002", |b| {
        b.iter(|| sum_of_even_value_fibs(black_box(4_000_000)))
    });
    c.bench_function("p003", |b| {
        b.iter(|| largest_prime_factor(black_box(600_851_475_143)))
    });
    c.bench_function("p004", |b| {
        b.iter(|| largest_palindrome(black_box(100), black_box(999)))
    });
    c.bench(
        "p005",
        ParameterizedBenchmark::new(
            "Lowest common multiple",
            |b, i| b.iter(|| lcm(*i)),
            vec![5, 10, 20, 30, 50, 80],
        )
        .with_function("Minimal prime factorization", |b, i| {
            b.iter(|| product_of_minimal_prime_factors(*i))
        }),
    );
    c.bench_function("p006", |b| b.iter(|| difference(black_box(100))));
}

criterion_main!(benches);
criterion_group! {
    name = benches;
    config = Criterion::default().noise_threshold(0.1);
    targets = benchmark
}
