use criterion::{criterion_group, criterion_main};
use criterion::{AxisScale, BenchmarkId, Criterion, PlotConfiguration};

criterion_main!(benches);
criterion_group! {
    name = benches;
    config = Criterion::default().noise_threshold(0.1);
    targets = benchmark
}

fn benchmark(c: &mut Criterion) {
    macro_rules! bench {
        ( $problem_id:ident, $group:expr, $axis_scale:ident, $( $inputs:expr )*, $( $method:ident -> $method_description:expr )* ) => {
            let mut $problem_id = c.benchmark_group($group);

            let plot_config = PlotConfiguration::default().summary_scale(AxisScale::$axis_scale);
            $problem_id.plot_config(plot_config);

            for size in [ $( $inputs, )* ].iter() {
                $(
                    $problem_id.bench_with_input(
                        BenchmarkId::new($method_description, size),
                        size,
                        |b, i| b.iter(|| euler_solutions::solutions::$problem_id::$method(*i))
                    );
                )*
            }

            $problem_id.finish();
        };
    }

    bench!(p001, "p001", Logarithmic,
           100 500 1_000 5_000 10_000,
           iterative -> "iterative"
           closed -> "closed form"
    );
}
