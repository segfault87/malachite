use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_q::test_util::bench::bucketers::{
    quadruple_1_rational_bit_bucketer, triple_1_rational_bit_bucketer,
};
use malachite_q::test_util::generators::{
    rational_rational_natural_natural_quadruple_gen_var_1, rational_rational_natural_triple_gen,
    rational_rational_natural_triple_gen_var_1,
};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_mutate_numerator);
    register_demo!(runner, demo_rational_mutate_denominator);
    register_demo!(runner, demo_rational_mutate_numerator_and_denominator);

    register_bench!(runner, benchmark_rational_mutate_numerator);
    register_bench!(runner, benchmark_rational_mutate_denominator);
    register_bench!(runner, benchmark_rational_mutate_numerator_and_denominator);
}

fn demo_rational_mutate_numerator(gm: GenMode, config: GenConfig, limit: usize) {
    for (mut q, out, new_numerator) in rational_rational_natural_triple_gen()
        .get(gm, &config)
        .take(limit)
    {
        let old_q = q.clone();
        let old_out = out.clone();
        let old_new_numerator = new_numerator.clone();
        let actual_out = q.mutate_numerator(|x| {
            *x = new_numerator;
            out
        });
        println!(
            "q := {}; q.mutate_numerator(|x| {{ *x = {}; {} }}) = {}; q = {}",
            old_q, old_new_numerator, old_out, actual_out, q
        );
    }
}

fn demo_rational_mutate_denominator(gm: GenMode, config: GenConfig, limit: usize) {
    for (mut q, out, new_denominator) in rational_rational_natural_triple_gen_var_1()
        .get(gm, &config)
        .take(limit)
    {
        let old_q = q.clone();
        let old_out = out.clone();
        let old_new_denominator = new_denominator.clone();
        let actual_out = q.mutate_denominator(|x| {
            *x = new_denominator;
            out
        });
        println!(
            "q := {}; q.mutate_denominator(|x| {{ *x = {}; {} }}) = {}; q = {}",
            old_q, old_new_denominator, old_out, actual_out, q
        );
    }
}

fn demo_rational_mutate_numerator_and_denominator(gm: GenMode, config: GenConfig, limit: usize) {
    for (mut q, out, new_numerator, new_denominator) in
        rational_rational_natural_natural_quadruple_gen_var_1()
            .get(gm, &config)
            .take(limit)
    {
        let old_q = q.clone();
        let old_out = out.clone();
        let old_new_numerator = new_numerator.clone();
        let old_new_denominator = new_denominator.clone();
        let actual_out = q.mutate_numerator_and_denominator(|x, y| {
            *x = new_numerator;
            *y = new_denominator;
            out
        });
        println!(
            "q := {}; q.mutate_numerator_and_denominator(|x, y| {{ *x = {}; *y = {}; {} }}) = {}; \
            q = {}",
            old_q, old_new_numerator, old_new_denominator, old_out, actual_out, q
        );
    }
}

fn benchmark_rational_mutate_numerator(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational.mutate_numerator(FnOnce(&mut Natural) -> T)",
        BenchmarkType::Single,
        rational_rational_natural_triple_gen().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &triple_1_rational_bit_bucketer("x"),
        &mut [("Malachite", &mut |(mut n, out, new_numerator)| {
            no_out!(n.mutate_numerator(|x| {
                *x = new_numerator;
                out
            }))
        })],
    );
}

fn benchmark_rational_mutate_denominator(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational.mutate_denominator(FnOnce(&mut Natural) -> T)",
        BenchmarkType::Single,
        rational_rational_natural_triple_gen_var_1().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &triple_1_rational_bit_bucketer("x"),
        &mut [("Malachite", &mut |(mut n, out, new_denominator)| {
            no_out!(n.mutate_denominator(|x| {
                *x = new_denominator;
                out
            }))
        })],
    );
}

fn benchmark_rational_mutate_numerator_and_denominator(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational.mutate_numerator_and_denominator(FnOnce(&mut Natural, &mut Natural) -> T)",
        BenchmarkType::Single,
        rational_rational_natural_natural_quadruple_gen_var_1().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_rational_bit_bucketer("x"),
        &mut [("Malachite", &mut |(
            mut n,
            out,
            new_numerator,
            new_denominator,
        )| {
            no_out!(n.mutate_numerator_and_denominator(|x, y| {
                *x = new_numerator;
                *y = new_denominator;
                out
            }))
        })],
    );
}
