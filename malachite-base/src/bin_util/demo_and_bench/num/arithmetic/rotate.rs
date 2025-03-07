use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::bench::bucketers::pair_1_bit_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{signed_unsigned_pair_gen, unsigned_pair_gen};
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_rotate_left_assign_unsigned);
    register_signed_demos!(runner, demo_rotate_left_assign_signed);
    register_unsigned_demos!(runner, demo_rotate_right_assign_unsigned);
    register_signed_demos!(runner, demo_rotate_right_assign_signed);

    register_unsigned_benches!(runner, benchmark_rotate_left_assign_unsigned);
    register_signed_benches!(runner, benchmark_rotate_left_assign_signed);
    register_unsigned_benches!(runner, benchmark_rotate_right_assign_unsigned);
    register_signed_benches!(runner, benchmark_rotate_right_assign_signed);
}

fn demo_rotate_left_assign_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) {
    for (mut u, n) in unsigned_pair_gen::<T, u64>().get(gm, &config).take(limit) {
        let old_u = u;
        u.rotate_left_assign(n);
        println!("i := {}; i.rotate_left_assign({}); i = {}", old_u, n, u);
    }
}

fn demo_rotate_left_assign_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) {
    for (mut i, n) in signed_unsigned_pair_gen::<T, u64>()
        .get(gm, &config)
        .take(limit)
    {
        let old_i = i;
        i.rotate_left_assign(n);
        println!("i := {}; i.rotate_left_assign({}); i = {}", old_i, n, i);
    }
}

fn demo_rotate_right_assign_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) {
    for (mut u, n) in unsigned_pair_gen::<T, u64>().get(gm, &config).take(limit) {
        let old_u = u;
        u.rotate_right_assign(n);
        println!("i := {}; i.rotate_right_assign({}); i = {}", old_u, n, u);
    }
}

fn demo_rotate_right_assign_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) {
    for (mut i, n) in signed_unsigned_pair_gen::<T, u64>()
        .get(gm, &config)
        .take(limit)
    {
        let old_i = i;
        i.rotate_right_assign(n);
        println!("i := {}; i.rotate_right_assign({}); i = {}", old_i, n, i);
    }
}

fn benchmark_rotate_left_assign_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.rotate_left_assign(u64)", T::NAME),
        BenchmarkType::Single,
        unsigned_pair_gen::<T, u64>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_bit_bucketer("u"),
        &mut [("Malachite", &mut |(mut u, n)| u.rotate_left_assign(n))],
    );
}

fn benchmark_rotate_left_assign_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.rotate_left_assign(u64)", T::NAME),
        BenchmarkType::Single,
        signed_unsigned_pair_gen::<T, u64>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_bit_bucketer("i"),
        &mut [("Malachite", &mut |(mut i, n)| i.rotate_left_assign(n))],
    );
}

fn benchmark_rotate_right_assign_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.rotate_right_assign(u64)", T::NAME),
        BenchmarkType::Single,
        unsigned_pair_gen::<T, u64>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_bit_bucketer("u"),
        &mut [("Malachite", &mut |(mut u, n)| u.rotate_right_assign(n))],
    );
}

fn benchmark_rotate_right_assign_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.rotate_right_assign(u64)", T::NAME),
        BenchmarkType::Single,
        signed_unsigned_pair_gen::<T, u64>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_bit_bucketer("i"),
        &mut [("Malachite", &mut |(mut i, n)| i.rotate_right_assign(n))],
    );
}
