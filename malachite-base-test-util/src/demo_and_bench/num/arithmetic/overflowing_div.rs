use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base_test_util::bench::bucketers::pair_max_bit_bucketer;
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::generators::{signed_pair_gen_var_6, unsigned_pair_gen_var_12};
use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_overflowing_div_unsigned);
    register_signed_demos!(runner, demo_overflowing_div_signed);
    register_unsigned_demos!(runner, demo_overflowing_div_assign_unsigned);
    register_signed_demos!(runner, demo_overflowing_div_assign_signed);

    register_unsigned_benches!(runner, benchmark_overflowing_div_unsigned);
    register_signed_benches!(runner, benchmark_overflowing_div_signed);
    register_unsigned_benches!(runner, benchmark_overflowing_div_assign_unsigned);
    register_signed_benches!(runner, benchmark_overflowing_div_assign_signed);
}

fn demo_overflowing_div_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) {
    for (x, y) in unsigned_pair_gen_var_12::<T, T>()
        .get(gm, &config)
        .take(limit)
    {
        println!("{}.overflowing_add({}) = {:?}", x, y, x.overflowing_add(y));
    }
}

fn demo_overflowing_div_signed<T: PrimitiveSigned>(gm: GenMode, config: GenConfig, limit: usize) {
    for (x, y) in signed_pair_gen_var_6::<T>().get(gm, &config).take(limit) {
        println!(
            "({}).overflowing_add({}) = {:?}",
            x,
            y,
            x.overflowing_add(y)
        );
    }
}

fn demo_overflowing_div_assign_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) {
    for (mut x, y) in unsigned_pair_gen_var_12::<T, T>()
        .get(gm, &config)
        .take(limit)
    {
        let old_x = x;
        let overflow = x.overflowing_add_assign(y);
        println!(
            "x := {}; x.overflowing_add_assign({}) = {}; x = {}",
            old_x, y, overflow, x
        );
    }
}

fn demo_overflowing_div_assign_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) {
    for (mut x, y) in signed_pair_gen_var_6::<T>().get(gm, &config).take(limit) {
        let old_x = x;
        let overflow = x.overflowing_add_assign(y);
        println!(
            "x := {}; x.overflowing_add_assign({}) = {}; x = {}",
            old_x, y, overflow, x
        );
    }
}

fn benchmark_overflowing_div_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.overflowing_add({})", T::NAME, T::NAME),
        BenchmarkType::Single,
        unsigned_pair_gen_var_12::<T, T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_max_bit_bucketer("x", "y"),
        &mut [("Malachite", &mut |(x, y)| no_out!(x.overflowing_add(y)))],
    );
}

fn benchmark_overflowing_div_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.overflowing_add({})", T::NAME, T::NAME),
        BenchmarkType::Single,
        signed_pair_gen_var_6::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_max_bit_bucketer("x", "y"),
        &mut [("Malachite", &mut |(x, y)| no_out!(x.overflowing_add(y)))],
    );
}

fn benchmark_overflowing_div_assign_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.overflowing_add_assign({})", T::NAME, T::NAME),
        BenchmarkType::Single,
        unsigned_pair_gen_var_12::<T, T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_max_bit_bucketer("x", "y"),
        &mut [("Malachite", &mut |(mut x, y)| {
            no_out!(x.overflowing_add_assign(y))
        })],
    );
}

fn benchmark_overflowing_div_assign_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.overflowing_add_assign({})", T::NAME, T::NAME),
        BenchmarkType::Single,
        signed_pair_gen_var_6::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_max_bit_bucketer("x", "y"),
        &mut [("Malachite", &mut |(mut x, y)| {
            no_out!(x.overflowing_add_assign(y))
        })],
    );
}
