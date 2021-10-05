use crate::bench::bucketers::integer_bit_bucketer;
use malachite_base::num::conversion::traits::IsInteger;
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::runner::Runner;
use malachite_nz_test_util::generators::integer_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_is_integer);
    register_bench!(runner, benchmark_integer_is_integer);
}

fn demo_integer_is_integer(gm: GenMode, config: GenConfig, limit: usize) {
    for n in integer_gen().get(gm, &config).take(limit) {
        if n.is_integer() {
            println!("{} is an integer", n);
        } else {
            println!("{} is not an integer", n);
        }
    }
}

fn benchmark_integer_is_integer(gm: GenMode, config: GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "Integer.is_integer()",
        BenchmarkType::Single,
        integer_gen().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &integer_bit_bucketer("x"),
        &mut [("Malachite", &mut |x| no_out!(x.is_integer()))],
    );
}