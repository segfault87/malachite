use malachite_base::iterators::prefix_to_string;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_q::arithmetic::traits::DenominatorsInClosedInterval;
use malachite_q::test_util::bench::bucketers::pair_2_rational_bit_bucketer;
use malachite_q::test_util::generators::rational_pair_gen_var_3;
use malachite_q::Rational;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_denominators_in_closed_interval);
    register_bench!(runner, benchmark_denominators_in_closed_interval);
}

fn demo_denominators_in_closed_interval(gm: GenMode, config: GenConfig, limit: usize) {
    for (a, b) in rational_pair_gen_var_3().get(gm, &config).take(limit) {
        println!(
            "denominators_in_closed_interval({}, {}) = {}",
            a,
            b,
            prefix_to_string(Rational::denominators_in_closed_interval(&a, &b), 20)
        );
    }
}

fn benchmark_denominators_in_closed_interval(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "denominators_in_closed_interval(&Rational, &Rational)",
        BenchmarkType::Single,
        rational_pair_gen_var_3().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_2_rational_bit_bucketer("x"),
        &mut [("Malachite", &mut |(a, b)| {
            no_out!(Rational::denominators_in_closed_interval(&a, &b))
        })],
    );
}
