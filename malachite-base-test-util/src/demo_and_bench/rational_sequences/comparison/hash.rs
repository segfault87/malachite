use malachite_base_test_util::bench::bucketers::rational_sequence_len_bucketer;
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::generators::unsigned_rational_sequence_gen;
use malachite_base_test_util::hash::hash;
use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_sequence_hash);
    register_bench!(runner, benchmark_rational_sequence_hash);
}

fn demo_rational_sequence_hash(gm: GenMode, config: GenConfig, limit: usize) {
    for xs in unsigned_rational_sequence_gen::<u8>()
        .get(gm, &config)
        .take(limit)
    {
        println!("hash({}) = {}", xs, hash(&xs));
    }
}

fn benchmark_rational_sequence_hash(gm: GenMode, config: GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "Rational hash",
        BenchmarkType::Single,
        unsigned_rational_sequence_gen::<u8>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &rational_sequence_len_bucketer("xs"),
        &mut [("Malachite", &mut |xs| no_out!(hash(&xs)))],
    );
}
