use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::runner::Runner;
use malachite_nz_test_util::bench::bucketers::integer_bit_bucketer;
use malachite_nz_test_util::generators::integer_gen;
use malachite_q::Rational;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_from_integer);
    register_demo!(runner, demo_rational_from_integer_ref);
    register_bench!(runner, benchmark_rational_from_integer_evaluation_strategy);
}

fn demo_rational_from_integer(gm: GenMode, config: GenConfig, limit: usize) {
    for n in integer_gen().get(gm, &config).take(limit) {
        let n_clone = n.clone();
        println!("Rational::from({}) = {}", n_clone, Rational::from(n));
    }
}

fn demo_rational_from_integer_ref(gm: GenMode, config: GenConfig, limit: usize) {
    for n in integer_gen().get(gm, &config).take(limit) {
        println!("Rational::from(&{}) = {}", n, Rational::from(&n));
    }
}

#[allow(unused_must_use)]
fn benchmark_rational_from_integer_evaluation_strategy(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational::from(Integer)",
        BenchmarkType::EvaluationStrategy,
        integer_gen().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &integer_bit_bucketer("n"),
        &mut [
            ("Rational::from(Integer)", &mut |n| {
                no_out!(Rational::from(n))
            }),
            ("Rational::from(&Integer)", &mut |n| {
                no_out!(Rational::from(&n))
            }),
        ],
    );
}
