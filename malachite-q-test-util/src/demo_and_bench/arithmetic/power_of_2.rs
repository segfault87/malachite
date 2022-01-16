use malachite_base::num::arithmetic::traits::PowerOf2;
use malachite_base_test_util::bench::bucketers::{signed_abs_bucketer, unsigned_direct_bucketer};
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::generators::{signed_gen_var_5, unsigned_gen_var_5};
use malachite_base_test_util::runner::Runner;
use malachite_q::Rational;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_power_of_2_u64);
    register_demo!(runner, demo_rational_power_of_2_i64);

    register_bench!(runner, benchmark_rational_power_of_2_u64);
    register_bench!(runner, benchmark_rational_power_of_2_i64);
}

fn demo_rational_power_of_2_u64(gm: GenMode, config: GenConfig, limit: usize) {
    for x in unsigned_gen_var_5::<u64>().get(gm, &config).take(limit) {
        println!("Rational::power_of_2({}) = {}", x, Rational::power_of_2(x));
    }
}

fn demo_rational_power_of_2_i64(gm: GenMode, config: GenConfig, limit: usize) {
    for x in signed_gen_var_5::<i64>().get(gm, &config).take(limit) {
        println!("Rational::power_of_2({}) = {}", x, Rational::power_of_2(x));
    }
}

fn benchmark_rational_power_of_2_u64(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational.power_of_2(u64)",
        BenchmarkType::Single,
        unsigned_gen_var_5::<u64>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &unsigned_direct_bucketer(),
        &mut [("Malachite", &mut |x| no_out!(Rational::power_of_2(x)))],
    );
}

fn benchmark_rational_power_of_2_i64(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational.power_of_2(i64)",
        BenchmarkType::Single,
        signed_gen_var_5::<i64>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &signed_abs_bucketer("x"),
        &mut [("Malachite", &mut |x| no_out!(Rational::power_of_2(x)))],
    );
}
