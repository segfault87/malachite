use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::runner::Runner;
use malachite_q::Rational;
use malachite_q_test_util::bench::bucketers::pair_1_rational_bit_bucketer;
use malachite_q_test_util::generators::{rational_signed_pair_gen, rational_unsigned_pair_gen};
use std::cmp::Ordering;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_rational_partial_cmp_abs_unsigned);
    register_signed_demos!(runner, demo_rational_partial_cmp_abs_signed);
    register_unsigned_demos!(runner, demo_unsigned_partial_cmp_abs_rational);
    register_signed_demos!(runner, demo_signed_partial_cmp_abs_rational);
    register_unsigned_demos!(runner, demo_rational_lt_abs_unsigned);
    register_signed_demos!(runner, demo_rational_lt_abs_signed);
    register_unsigned_demos!(runner, demo_rational_gt_abs_unsigned);
    register_signed_demos!(runner, demo_rational_gt_abs_signed);
    register_unsigned_demos!(runner, demo_rational_le_abs_unsigned);
    register_signed_demos!(runner, demo_rational_le_abs_signed);
    register_unsigned_demos!(runner, demo_rational_ge_abs_unsigned);
    register_signed_demos!(runner, demo_rational_ge_abs_signed);
    register_unsigned_demos!(runner, demo_unsigned_lt_abs_rational);
    register_signed_demos!(runner, demo_signed_lt_abs_rational);
    register_unsigned_demos!(runner, demo_unsigned_gt_abs_rational);
    register_signed_demos!(runner, demo_signed_gt_abs_rational);
    register_unsigned_demos!(runner, demo_unsigned_le_abs_rational);
    register_signed_demos!(runner, demo_signed_le_abs_rational);
    register_unsigned_demos!(runner, demo_unsigned_ge_abs_rational);
    register_signed_demos!(runner, demo_signed_ge_abs_rational);

    register_unsigned_benches!(runner, benchmark_rational_partial_cmp_abs_unsigned);
    register_signed_benches!(runner, benchmark_rational_partial_cmp_abs_signed);
    register_unsigned_benches!(runner, benchmark_unsigned_partial_cmp_abs_rational);
    register_signed_benches!(runner, benchmark_signed_partial_cmp_abs_rational);
    register_unsigned_benches!(runner, benchmark_rational_lt_abs_unsigned);
    register_signed_benches!(runner, benchmark_rational_lt_abs_signed);
    register_unsigned_benches!(runner, benchmark_rational_gt_abs_unsigned);
    register_signed_benches!(runner, benchmark_rational_gt_abs_signed);
    register_unsigned_benches!(runner, benchmark_rational_le_abs_unsigned);
    register_signed_benches!(runner, benchmark_rational_le_abs_signed);
    register_unsigned_benches!(runner, benchmark_rational_ge_abs_unsigned);
    register_signed_benches!(runner, benchmark_rational_ge_abs_signed);
    register_unsigned_benches!(runner, benchmark_unsigned_lt_abs_rational);
    register_signed_benches!(runner, benchmark_signed_lt_abs_rational);
    register_unsigned_benches!(runner, benchmark_unsigned_gt_abs_rational);
    register_signed_benches!(runner, benchmark_signed_gt_abs_rational);
    register_unsigned_benches!(runner, benchmark_unsigned_le_abs_rational);
    register_signed_benches!(runner, benchmark_signed_le_abs_rational);
    register_unsigned_benches!(runner, benchmark_unsigned_ge_abs_rational);
    register_signed_benches!(runner, benchmark_signed_ge_abs_rational);
}

fn demo_rational_partial_cmp_abs_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) where
    Rational: PartialOrdAbs<T>,
{
    for (n, u) in rational_unsigned_pair_gen::<T>()
        .get(gm, &config)
        .take(limit)
    {
        match n.partial_cmp_abs(&u).unwrap() {
            Ordering::Less => println!("|{}| < |{}|", n, u),
            Ordering::Equal => println!("|{}| = |{}|", n, u),
            Ordering::Greater => println!("|{}| > |{}|", n, u),
        }
    }
}

fn demo_rational_partial_cmp_abs_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) where
    Rational: PartialOrdAbs<T>,
{
    for (n, i) in rational_signed_pair_gen::<T>().get(gm, &config).take(limit) {
        match n.partial_cmp_abs(&i).unwrap() {
            Ordering::Less => println!("|{}| < |{}|", n, i),
            Ordering::Equal => println!("|{}| = |{}|", n, i),
            Ordering::Greater => println!("|{}| > |{}|", n, i),
        }
    }
}

fn demo_unsigned_partial_cmp_abs_rational<T: PartialOrdAbs<Rational> + PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) {
    for (n, u) in rational_unsigned_pair_gen::<T>()
        .get(gm, &config)
        .take(limit)
    {
        match u.partial_cmp_abs(&n).unwrap() {
            Ordering::Less => println!("|{}| < |{}|", u, n),
            Ordering::Equal => println!("|{}| = |{}|", u, n),
            Ordering::Greater => println!("|{}| > |{}|", u, n),
        }
    }
}

fn demo_signed_partial_cmp_abs_rational<T: PartialOrdAbs<Rational> + PrimitiveSigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) {
    for (n, i) in rational_signed_pair_gen::<T>().get(gm, &config).take(limit) {
        match i.partial_cmp_abs(&n).unwrap() {
            Ordering::Less => println!("|{}| < |{}|", i, n),
            Ordering::Equal => println!("|{}| = |{}|", i, n),
            Ordering::Greater => println!("|{}| > |{}|", i, n),
        }
    }
}

fn demo_rational_lt_abs_unsigned<T: PrimitiveUnsigned>(gm: GenMode, config: GenConfig, limit: usize)
where
    Rational: PartialOrdAbs<T>,
{
    for (n, u) in rational_unsigned_pair_gen::<T>()
        .get(gm, &config)
        .take(limit)
    {
        if n.lt_abs(&u) {
            println!("|{}| < |{}|", n, u);
        } else {
            println!("|{}| ≮ |{}|", n, u);
        }
    }
}

fn demo_rational_lt_abs_signed<T: PrimitiveSigned>(gm: GenMode, config: GenConfig, limit: usize)
where
    Rational: PartialOrdAbs<T>,
{
    for (n, i) in rational_signed_pair_gen::<T>().get(gm, &config).take(limit) {
        if n.lt_abs(&i) {
            println!("|{}| < |{}|", n, i);
        } else {
            println!("|{}| ≮ |{}|", n, i);
        }
    }
}

fn demo_rational_gt_abs_unsigned<T: PrimitiveUnsigned>(gm: GenMode, config: GenConfig, limit: usize)
where
    Rational: PartialOrdAbs<T>,
{
    for (n, u) in rational_unsigned_pair_gen::<T>()
        .get(gm, &config)
        .take(limit)
    {
        if n.gt_abs(&u) {
            println!("|{}| > |{}|", n, u);
        } else {
            println!("|{}| ≯ |{}|", n, u);
        }
    }
}

fn demo_rational_gt_abs_signed<T: PrimitiveSigned>(gm: GenMode, config: GenConfig, limit: usize)
where
    Rational: PartialOrdAbs<T>,
{
    for (n, i) in rational_signed_pair_gen::<T>().get(gm, &config).take(limit) {
        if n.gt_abs(&i) {
            println!("|{}| > |{}|", n, i);
        } else {
            println!("|{}| ≯ |{}|", n, i);
        }
    }
}

fn demo_rational_le_abs_unsigned<T: PrimitiveUnsigned>(gm: GenMode, config: GenConfig, limit: usize)
where
    Rational: PartialOrdAbs<T>,
{
    for (n, u) in rational_unsigned_pair_gen::<T>()
        .get(gm, &config)
        .take(limit)
    {
        if n.le_abs(&u) {
            println!("|{}| ≤ |{}|", n, u);
        } else {
            println!("|{}| ≰ |{}|", n, u);
        }
    }
}

fn demo_rational_le_abs_signed<T: PrimitiveSigned>(gm: GenMode, config: GenConfig, limit: usize)
where
    Rational: PartialOrdAbs<T>,
{
    for (n, i) in rational_signed_pair_gen::<T>().get(gm, &config).take(limit) {
        if n.le_abs(&i) {
            println!("|{}| ≤ |{}|", n, i);
        } else {
            println!("|{}| ≰ |{}|", n, i);
        }
    }
}

fn demo_rational_ge_abs_unsigned<T: PrimitiveUnsigned>(gm: GenMode, config: GenConfig, limit: usize)
where
    Rational: PartialOrdAbs<T>,
{
    for (n, u) in rational_unsigned_pair_gen::<T>()
        .get(gm, &config)
        .take(limit)
    {
        if n.ge_abs(&u) {
            println!("|{}| ≥ |{}|", n, u);
        } else {
            println!("|{}| ≱ |{}|", n, u);
        }
    }
}

fn demo_rational_ge_abs_signed<T: PrimitiveSigned>(gm: GenMode, config: GenConfig, limit: usize)
where
    Rational: PartialOrdAbs<T>,
{
    for (n, i) in rational_signed_pair_gen::<T>().get(gm, &config).take(limit) {
        if n.ge_abs(&i) {
            println!("|{}| ≥ |{}|", n, i);
        } else {
            println!("|{}| ≱ |{}|", n, i);
        }
    }
}

fn demo_unsigned_lt_abs_rational<T: PartialOrdAbs<Rational> + PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) {
    for (n, u) in rational_unsigned_pair_gen::<T>()
        .get(gm, &config)
        .take(limit)
    {
        if u.lt_abs(&n) {
            println!("|{}| < |{}|", u, n);
        } else {
            println!("|{}| ≮ |{}|", u, n);
        }
    }
}

fn demo_signed_lt_abs_rational<T: PartialOrdAbs<Rational> + PrimitiveSigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) {
    for (n, i) in rational_signed_pair_gen::<T>().get(gm, &config).take(limit) {
        if i.lt_abs(&n) {
            println!("|{}| < |{}|", i, n);
        } else {
            println!("|{}| ≮ |{}|", i, n);
        }
    }
}

fn demo_unsigned_gt_abs_rational<T: PartialOrdAbs<Rational> + PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) {
    for (n, u) in rational_unsigned_pair_gen::<T>()
        .get(gm, &config)
        .take(limit)
    {
        if u.gt_abs(&n) {
            println!("|{}| > |{}|", u, n);
        } else {
            println!("|{}| ≯ |{}|", u, n);
        }
    }
}

fn demo_signed_gt_abs_rational<T: PartialOrdAbs<Rational> + PrimitiveSigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) {
    for (n, i) in rational_signed_pair_gen::<T>().get(gm, &config).take(limit) {
        if i.gt_abs(&n) {
            println!("|{}| > |{}|", i, n);
        } else {
            println!("|{}| ≯ |{}|", i, n);
        }
    }
}

fn demo_unsigned_le_abs_rational<T: PartialOrdAbs<Rational> + PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) {
    for (n, u) in rational_unsigned_pair_gen::<T>()
        .get(gm, &config)
        .take(limit)
    {
        if u.le_abs(&n) {
            println!("|{}| ≤ |{}|", u, n);
        } else {
            println!("|{}| ≰ |{}|", u, n);
        }
    }
}

fn demo_signed_le_abs_rational<T: PartialOrdAbs<Rational> + PrimitiveSigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) {
    for (n, i) in rational_signed_pair_gen::<T>().get(gm, &config).take(limit) {
        if i.le_abs(&n) {
            println!("|{}| ≤ |{}|", i, n);
        } else {
            println!("|{}| ≰ |{}|", i, n);
        }
    }
}

fn demo_unsigned_ge_abs_rational<T: PartialOrdAbs<Rational> + PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) {
    for (n, u) in rational_unsigned_pair_gen::<T>()
        .get(gm, &config)
        .take(limit)
    {
        if u.ge_abs(&n) {
            println!("|{}| ≥ |{}|", u, n);
        } else {
            println!("|{}| ≱ |{}|", u, n);
        }
    }
}

fn demo_signed_ge_abs_rational<T: PartialOrdAbs<Rational> + PrimitiveSigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) {
    for (n, i) in rational_signed_pair_gen::<T>().get(gm, &config).take(limit) {
        if i.ge_abs(&n) {
            println!("|{}| ≥ |{}|", i, n);
        } else {
            println!("|{}| ≱ |{}|", i, n);
        }
    }
}

fn benchmark_rational_partial_cmp_abs_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) where
    Rational: PartialOrdAbs<T>,
{
    run_benchmark(
        &format!("Rational.partial_cmp_abs(&{})", T::NAME),
        BenchmarkType::Single,
        rational_unsigned_pair_gen::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_bit_bucketer("x"),
        &mut [("Malachite", &mut |(x, y)| no_out!(x.partial_cmp_abs(&y)))],
    );
}

fn benchmark_rational_partial_cmp_abs_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) where
    Rational: PartialOrdAbs<T>,
{
    run_benchmark(
        &format!("Rational.partial_cmp_abs(&{})", T::NAME),
        BenchmarkType::Single,
        rational_signed_pair_gen::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_bit_bucketer("x"),
        &mut [("Malachite", &mut |(x, y)| no_out!(x.partial_cmp_abs(&y)))],
    );
}

fn benchmark_unsigned_partial_cmp_abs_rational<T: PartialOrdAbs<Rational> + PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.partial_cmp_abs(&Rational)", T::NAME),
        BenchmarkType::Single,
        rational_unsigned_pair_gen::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_bit_bucketer("x"),
        &mut [("Malachite", &mut |(x, y)| no_out!(y.partial_cmp_abs(&x)))],
    );
}

fn benchmark_signed_partial_cmp_abs_rational<T: PartialOrdAbs<Rational> + PrimitiveSigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.partial_cmp_abs(&Rational)", T::NAME),
        BenchmarkType::Single,
        rational_signed_pair_gen::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_bit_bucketer("x"),
        &mut [("Malachite", &mut |(x, y)| no_out!(y.partial_cmp_abs(&x)))],
    );
}

fn benchmark_rational_lt_abs_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) where
    Rational: PartialOrdAbs<T>,
{
    run_benchmark(
        &format!("Rational.lt_abs(&{})", T::NAME),
        BenchmarkType::Single,
        rational_unsigned_pair_gen::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_bit_bucketer("x"),
        &mut [("Malachite", &mut |(x, y)| no_out!(x.lt_abs(&y)))],
    );
}

fn benchmark_rational_lt_abs_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) where
    Rational: PartialOrdAbs<T>,
{
    run_benchmark(
        &format!("Rational.lt_abs(&{})", T::NAME),
        BenchmarkType::Single,
        rational_signed_pair_gen::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_bit_bucketer("x"),
        &mut [("Malachite", &mut |(x, y)| no_out!(x.lt_abs(&y)))],
    );
}

fn benchmark_rational_gt_abs_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) where
    Rational: PartialOrdAbs<T>,
{
    run_benchmark(
        &format!("Rational.gt_abs(&{})", T::NAME),
        BenchmarkType::Single,
        rational_unsigned_pair_gen::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_bit_bucketer("x"),
        &mut [("Malachite", &mut |(x, y)| no_out!(x.gt_abs(&y)))],
    );
}

fn benchmark_rational_gt_abs_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) where
    Rational: PartialOrdAbs<T>,
{
    run_benchmark(
        &format!("Rational.gt_abs(&{})", T::NAME),
        BenchmarkType::Single,
        rational_signed_pair_gen::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_bit_bucketer("x"),
        &mut [("Malachite", &mut |(x, y)| no_out!(x.gt_abs(&y)))],
    );
}

fn benchmark_rational_le_abs_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) where
    Rational: PartialOrdAbs<T>,
{
    run_benchmark(
        &format!("Rational.le_abs(&{})", T::NAME),
        BenchmarkType::Single,
        rational_unsigned_pair_gen::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_bit_bucketer("x"),
        &mut [("Malachite", &mut |(x, y)| no_out!(x.le_abs(&y)))],
    );
}

fn benchmark_rational_le_abs_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) where
    Rational: PartialOrdAbs<T>,
{
    run_benchmark(
        &format!("Rational.le_abs(&{})", T::NAME),
        BenchmarkType::Single,
        rational_signed_pair_gen::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_bit_bucketer("x"),
        &mut [("Malachite", &mut |(x, y)| no_out!(x.le_abs(&y)))],
    );
}

fn benchmark_rational_ge_abs_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) where
    Rational: PartialOrdAbs<T>,
{
    run_benchmark(
        &format!("Rational.ge_abs(&{})", T::NAME),
        BenchmarkType::Single,
        rational_unsigned_pair_gen::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_bit_bucketer("x"),
        &mut [("Malachite", &mut |(x, y)| no_out!(x.ge_abs(&y)))],
    );
}

fn benchmark_rational_ge_abs_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) where
    Rational: PartialOrdAbs<T>,
{
    run_benchmark(
        &format!("Rational.ge_abs(&{})", T::NAME),
        BenchmarkType::Single,
        rational_signed_pair_gen::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_bit_bucketer("x"),
        &mut [("Malachite", &mut |(x, y)| no_out!(x.ge_abs(&y)))],
    );
}

fn benchmark_unsigned_lt_abs_rational<T: PartialOrdAbs<Rational> + PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.lt_abs(&Rational)", T::NAME),
        BenchmarkType::Single,
        rational_unsigned_pair_gen::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_bit_bucketer("x"),
        &mut [("Malachite", &mut |(x, y)| no_out!(y.lt_abs(&x)))],
    );
}

fn benchmark_signed_lt_abs_rational<T: PartialOrdAbs<Rational> + PrimitiveSigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.lt_abs(&Rational)", T::NAME),
        BenchmarkType::Single,
        rational_signed_pair_gen::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_bit_bucketer("x"),
        &mut [("Malachite", &mut |(x, y)| no_out!(y.lt_abs(&x)))],
    );
}

fn benchmark_unsigned_gt_abs_rational<T: PartialOrdAbs<Rational> + PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.gt_abs(&Rational)", T::NAME),
        BenchmarkType::Single,
        rational_unsigned_pair_gen::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_bit_bucketer("x"),
        &mut [("Malachite", &mut |(x, y)| no_out!(y.gt_abs(&x)))],
    );
}

fn benchmark_signed_gt_abs_rational<T: PartialOrdAbs<Rational> + PrimitiveSigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.gt_abs(&Rational)", T::NAME),
        BenchmarkType::Single,
        rational_signed_pair_gen::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_bit_bucketer("x"),
        &mut [("Malachite", &mut |(x, y)| no_out!(y.gt_abs(&x)))],
    );
}

fn benchmark_unsigned_le_abs_rational<T: PartialOrdAbs<Rational> + PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.le_abs(&Rational)", T::NAME),
        BenchmarkType::Single,
        rational_unsigned_pair_gen::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_bit_bucketer("x"),
        &mut [("Malachite", &mut |(x, y)| no_out!(y.le_abs(&x)))],
    );
}

fn benchmark_signed_le_abs_rational<T: PartialOrdAbs<Rational> + PrimitiveSigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.le_abs(&Rational)", T::NAME),
        BenchmarkType::Single,
        rational_signed_pair_gen::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_bit_bucketer("x"),
        &mut [("Malachite", &mut |(x, y)| no_out!(y.le_abs(&x)))],
    );
}

fn benchmark_unsigned_ge_abs_rational<T: PartialOrdAbs<Rational> + PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.ge_abs(&Rational)", T::NAME),
        BenchmarkType::Single,
        rational_unsigned_pair_gen::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_bit_bucketer("x"),
        &mut [("Malachite", &mut |(x, y)| no_out!(y.ge_abs(&x)))],
    );
}

fn benchmark_signed_ge_abs_rational<T: PartialOrdAbs<Rational> + PrimitiveSigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.ge_abs(&Rational)", T::NAME),
        BenchmarkType::Single,
        rational_signed_pair_gen::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_bit_bucketer("x"),
        &mut [("Malachite", &mut |(x, y)| no_out!(y.ge_abs(&x)))],
    );
}
