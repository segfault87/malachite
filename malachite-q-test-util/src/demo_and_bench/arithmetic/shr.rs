use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::runner::Runner;
use malachite_q::Rational;
use malachite_q_test_util::bench::bucketers::{
    pair_1_rational_bit_bucketer, pair_2_pair_1_rational_bit_bucketer,
};
use malachite_q_test_util::generators::{
    rational_signed_pair_gen_var_1, rational_signed_pair_gen_var_1_rm,
    rational_unsigned_pair_gen_var_1, rational_unsigned_pair_gen_var_1_rm,
};
use std::ops::{Shr, ShrAssign};

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_rational_shr_assign_unsigned);
    register_signed_demos!(runner, demo_rational_shr_assign_signed);
    register_unsigned_demos!(runner, demo_rational_shr_unsigned);
    register_signed_demos!(runner, demo_rational_shr_signed);
    register_unsigned_demos!(runner, demo_rational_shr_unsigned_ref);
    register_signed_demos!(runner, demo_rational_shr_signed_ref);

    register_unsigned_benches!(runner, benchmark_rational_shr_assign_unsigned);
    register_signed_benches!(runner, benchmark_rational_shr_assign_signed);
    register_unsigned_benches!(runner, benchmark_rational_shr_unsigned_evaluation_strategy);
    register_signed_benches!(runner, benchmark_rational_shr_signed_evaluation_strategy);

    register_bench!(runner, benchmark_rational_shr_assign_u32_library_comparison);
    register_bench!(runner, benchmark_rational_shr_u32_library_comparison);
    register_bench!(runner, benchmark_rational_shr_assign_i32_library_comparison);
    register_bench!(runner, benchmark_rational_shr_i32_library_comparison);
}

fn demo_rational_shr_assign_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) where
    Rational: ShrAssign<T>,
{
    for (mut n, u) in rational_unsigned_pair_gen_var_1::<T>()
        .get(gm, &config)
        .take(limit)
    {
        let n_old = n.clone();
        n >>= u;
        println!("x := {}; x >>= {}; x = {}", n_old, u, n);
    }
}

fn demo_rational_shr_assign_signed<T: PrimitiveSigned>(gm: GenMode, config: GenConfig, limit: usize)
where
    Rational: ShrAssign<T>,
{
    for (mut n, i) in rational_signed_pair_gen_var_1::<T>()
        .get(gm, &config)
        .take(limit)
    {
        let n_old = n.clone();
        n >>= i;
        println!("x := {}; x >>= {}; x = {}", n_old, i, n);
    }
}

fn demo_rational_shr_unsigned<T: PrimitiveUnsigned>(gm: GenMode, config: GenConfig, limit: usize)
where
    Rational: Shr<T, Output = Rational>,
{
    for (n, u) in rational_unsigned_pair_gen_var_1::<T>()
        .get(gm, &config)
        .take(limit)
    {
        let n_old = n.clone();
        println!("{} >> {} = {}", n_old, u, n >> u);
    }
}

fn demo_rational_shr_signed<T: PrimitiveSigned>(gm: GenMode, config: GenConfig, limit: usize)
where
    Rational: Shr<T, Output = Rational>,
{
    for (n, i) in rational_signed_pair_gen_var_1::<T>()
        .get(gm, &config)
        .take(limit)
    {
        let n_old = n.clone();
        println!("{} >> {} = {}", n_old, i, n >> i);
    }
}

fn demo_rational_shr_unsigned_ref<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) where
    for<'a> &'a Rational: Shr<T, Output = Rational>,
{
    for (n, u) in rational_unsigned_pair_gen_var_1::<T>()
        .get(gm, &config)
        .take(limit)
    {
        println!("&{} >> {} = {}", n, u, &n >> u);
    }
}

fn demo_rational_shr_signed_ref<T: PrimitiveSigned>(gm: GenMode, config: GenConfig, limit: usize)
where
    for<'a> &'a Rational: Shr<T, Output = Rational>,
{
    for (n, i) in rational_signed_pair_gen_var_1::<T>()
        .get(gm, &config)
        .take(limit)
    {
        println!("&{} >> {} = {}", n, i, &n >> i);
    }
}

fn benchmark_rational_shr_assign_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) where
    Rational: ShrAssign<T>,
{
    run_benchmark(
        &format!("Rational >>= {}", T::NAME),
        BenchmarkType::Single,
        rational_unsigned_pair_gen_var_1::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_bit_bucketer("n"),
        &mut [("Malachite", &mut |(mut n, u)| n >>= u)],
    );
}

fn benchmark_rational_shr_assign_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) where
    Rational: ShrAssign<T>,
{
    run_benchmark(
        &format!("Rational >>= {}", T::NAME),
        BenchmarkType::Single,
        rational_signed_pair_gen_var_1::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_bit_bucketer("n"),
        &mut [("Malachite", &mut |(mut n, i)| n >>= i)],
    );
}

#[allow(clippy::no_effect, unused_must_use)]
fn benchmark_rational_shr_unsigned_evaluation_strategy<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) where
    Rational: Shr<T, Output = Rational>,
    for<'a> &'a Rational: Shr<T, Output = Rational>,
{
    run_benchmark(
        &format!("Rational >> {}", T::NAME),
        BenchmarkType::EvaluationStrategy,
        rational_unsigned_pair_gen_var_1::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_bit_bucketer("n"),
        &mut [
            (&format!("Rational >> {}", T::NAME), &mut |(x, y)| {
                no_out!(x >> y)
            }),
            (&format!("&Rational >> {}", T::NAME), &mut |(x, y)| {
                no_out!(&x >> y)
            }),
        ],
    );
}

#[allow(clippy::no_effect, unused_must_use)]
fn benchmark_rational_shr_signed_evaluation_strategy<T: PrimitiveSigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) where
    Rational: Shr<T, Output = Rational>,
    for<'a> &'a Rational: Shr<T, Output = Rational>,
{
    run_benchmark(
        &format!("Rational >> {}", T::NAME),
        BenchmarkType::EvaluationStrategy,
        rational_signed_pair_gen_var_1::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_bit_bucketer("n"),
        &mut [
            (&format!("Rational >> {}", T::NAME), &mut |(x, y)| {
                no_out!(x >> y)
            }),
            (&format!("&Rational >> {}", T::NAME), &mut |(x, y)| {
                no_out!(&x >> y)
            }),
        ],
    );
}

fn benchmark_rational_shr_assign_u32_library_comparison(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational >>= u32",
        BenchmarkType::LibraryComparison,
        rational_unsigned_pair_gen_var_1_rm::<u32>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_2_pair_1_rational_bit_bucketer("n"),
        &mut [
            ("Malachite", &mut |(_, (mut x, y))| x >>= y),
            ("rug", &mut |((mut x, y), _)| x >>= y),
        ],
    );
}

#[allow(clippy::no_effect, clippy::unnecessary_operation, unused_must_use)]
fn benchmark_rational_shr_u32_library_comparison(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational >> u32",
        BenchmarkType::LibraryComparison,
        rational_unsigned_pair_gen_var_1_rm::<u32>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_2_pair_1_rational_bit_bucketer("n"),
        &mut [
            ("Malachite", &mut |(_, (x, y))| no_out!(x >> y)),
            ("rug", &mut |((x, y), _)| no_out!(x >> y)),
        ],
    );
}

fn benchmark_rational_shr_assign_i32_library_comparison(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational >>= i32",
        BenchmarkType::LibraryComparison,
        rational_signed_pair_gen_var_1_rm::<i32>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_2_pair_1_rational_bit_bucketer("n"),
        &mut [
            ("Malachite", &mut |(_, (mut x, y))| x >>= y),
            ("rug", &mut |((mut x, y), _)| x >>= y),
        ],
    );
}

#[allow(clippy::no_effect, clippy::unnecessary_operation, unused_must_use)]
fn benchmark_rational_shr_i32_library_comparison(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational >> i32",
        BenchmarkType::LibraryComparison,
        rational_signed_pair_gen_var_1_rm::<i32>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_2_pair_1_rational_bit_bucketer("n"),
        &mut [
            ("Malachite", &mut |(_, (x, y))| no_out!(x >> y)),
            ("rug", &mut |((x, y), _)| no_out!(x >> y)),
        ],
    );
}
