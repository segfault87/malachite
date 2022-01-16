use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::{CheckedFrom, ConvertibleFrom, RoundingFrom};
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::runner::Runner;
use malachite_q::Rational;
use malachite_q_test_util::bench::bucketers::{
    pair_1_rational_bit_bucketer, rational_bit_bucketer,
};
use malachite_q_test_util::generators::{rational_gen, rational_rounding_mode_pair_gen_var_3};

pub(crate) fn register(runner: &mut Runner) {
    register_primitive_int_demos!(runner, demo_primitive_int_checked_from_rational);
    register_primitive_int_demos!(runner, demo_primitive_int_convertible_from_rational);
    register_primitive_int_demos!(runner, demo_primitive_int_rounding_from_rational);

    register_primitive_int_benches!(runner, benchmark_primitive_int_checked_from_rational);
    register_primitive_int_benches!(runner, benchmark_primitive_int_convertible_from_rational);
    register_primitive_int_benches!(runner, benchmark_primitive_int_rounding_from_rational);
}

fn demo_primitive_int_checked_from_rational<T: for<'a> CheckedFrom<&'a Rational> + PrimitiveInt>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) {
    for x in rational_gen().get(gm, &config).take(limit) {
        println!(
            "{}::checked_from({}) = {:?}",
            T::NAME,
            x,
            T::checked_from(&x)
        );
    }
}

fn demo_primitive_int_convertible_from_rational<
    T: for<'a> ConvertibleFrom<&'a Rational> + PrimitiveInt,
>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) {
    for x in rational_gen().get(gm, &config).take(limit) {
        println!(
            "{} is {}convertible to a {}",
            x,
            if T::convertible_from(&x) { "" } else { "not " },
            T::NAME
        );
    }
}

fn demo_primitive_int_rounding_from_rational<
    T: for<'a> ConvertibleFrom<&'a Rational> + PrimitiveInt + for<'a> RoundingFrom<&'a Rational>,
>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) where
    Rational: PartialOrd<T>,
{
    for (x, rm) in rational_rounding_mode_pair_gen_var_3::<T>()
        .get(gm, &config)
        .take(limit)
    {
        println!(
            "{}::rounding_from({}, {}) = {}",
            T::NAME,
            x,
            rm,
            T::rounding_from(&x, rm)
        );
    }
}

fn benchmark_primitive_int_checked_from_rational<
    T: for<'a> CheckedFrom<&'a Rational> + PrimitiveInt,
>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}::checked_from(Rational)", T::NAME),
        BenchmarkType::Single,
        rational_gen().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &rational_bit_bucketer("x"),
        &mut [("Malachite", &mut |x| no_out!(T::checked_from(&x)))],
    );
}

fn benchmark_primitive_int_convertible_from_rational<
    T: for<'a> ConvertibleFrom<&'a Rational> + PrimitiveInt,
>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}::convertible_from(Rational)", T::NAME),
        BenchmarkType::Single,
        rational_gen().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &rational_bit_bucketer("x"),
        &mut [("Malachite", &mut |x| no_out!(T::convertible_from(&x)))],
    );
}

fn benchmark_primitive_int_rounding_from_rational<
    T: for<'a> ConvertibleFrom<&'a Rational> + PrimitiveInt + for<'a> RoundingFrom<&'a Rational>,
>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) where
    Rational: PartialOrd<T>,
{
    run_benchmark(
        &format!("{}::rounding_from(Rational)", T::NAME),
        BenchmarkType::Single,
        rational_rounding_mode_pair_gen_var_3::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_bit_bucketer("x"),
        &mut [("Malachite", &mut |(x, rm)| {
            no_out!(T::rounding_from(&x, rm))
        })],
    );
}
