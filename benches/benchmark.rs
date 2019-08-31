use criterion::{criterion_group, criterion_main};
use criterion::{BenchmarkId, Criterion};

use euler_solutions::p001::sum_all_multiples;
use euler_solutions::p002::sum_of_even_value_fibs;
use euler_solutions::p003::largest_prime_factor;
use euler_solutions::p004::largest_palindrome;
use euler_solutions::p005::{lcm, product_of_minimal_prime_factors};
use euler_solutions::p006::difference;

fn benchmark(c: &mut Criterion) {
    c.bench_function("p001", |b| b.iter(|| sum_all_multiples(&[3, 5], 1_000)));

    c.bench_function("p002", |b| b.iter(|| sum_of_even_value_fibs(4_000_000)));

    c.bench_function("p003", |b| b.iter(|| largest_prime_factor(600_851_475_143)));

    c.bench_function("p004", |b| b.iter(|| largest_palindrome(100, 999)));

    let mut p005 = c.benchmark_group("p005");
    for size in [5, 10, 20, 30, 50, 80].iter() {
        p005.bench_with_input(
            BenchmarkId::new("Lowest common multiple", size),
            size,
            |b, i| b.iter(|| lcm(*i)),
        );
        p005.bench_with_input(
            BenchmarkId::new("Multiple prime factorization", size),
            size,
            |b, i| b.iter(|| product_of_minimal_prime_factors(*i)),
        );
    }
    p005.finish();

    c.bench_function("p006", |b| b.iter(|| difference(100)));
}

criterion_main!(benches);
criterion_group! {
    name = benches;
    config = Criterion::default().noise_threshold(0.1);
    targets = benchmark
}
