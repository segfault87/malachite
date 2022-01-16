use malachite_base::strings::ToDebugString;
use malachite_base_test_util::bench::bucketers::rational_sequence_len_bucketer;
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::generators::unsigned_rational_sequence_gen;
use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_sequence_to_string);
    register_demo!(runner, demo_rational_sequence_to_debug_string);

    register_bench!(runner, benchmark_rational_sequence_to_string);
    register_bench!(runner, benchmark_rational_sequence_to_debug_string);
}

fn demo_rational_sequence_to_string(gm: GenMode, config: GenConfig, limit: usize) {
    for xs in unsigned_rational_sequence_gen::<u8>()
        .get(gm, &config)
        .take(limit)
    {
        println!("{}", xs);
    }
}

fn demo_rational_sequence_to_debug_string(gm: GenMode, config: GenConfig, limit: usize) {
    for xs in unsigned_rational_sequence_gen::<u8>()
        .get(gm, &config)
        .take(limit)
    {
        println!("{:?}", xs);
    }
}

fn benchmark_rational_sequence_to_string(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalSequence.to_string()",
        BenchmarkType::Single,
        unsigned_rational_sequence_gen::<u8>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &rational_sequence_len_bucketer("xs"),
        &mut [("Malachite", &mut |xs| no_out!(xs.to_string()))],
    );
}

fn benchmark_rational_sequence_to_debug_string(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalSequence.to_debug_string()",
        BenchmarkType::Single,
        unsigned_rational_sequence_gen::<u8>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &rational_sequence_len_bucketer("xs"),
        &mut [("Malachite", &mut |xs| no_out!(xs.to_debug_string()))],
    );
}
