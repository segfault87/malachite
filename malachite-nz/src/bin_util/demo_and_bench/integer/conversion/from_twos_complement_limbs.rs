use malachite_base::test_util::bench::bucketers::vec_len_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_vec_gen;
use malachite_base::test_util::runner::Runner;
use malachite_nz::integer::Integer;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_from_twos_complement_limbs_asc);
    register_demo!(runner, demo_integer_from_twos_complement_limbs_desc);
    register_demo!(runner, demo_integer_from_owned_twos_complement_limbs_asc);
    register_demo!(runner, demo_integer_from_owned_twos_complement_limbs_desc);

    register_bench!(
        runner,
        benchmark_integer_from_twos_complement_limbs_asc_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_integer_from_twos_complement_limbs_desc_evaluation_strategy
    );
}

fn demo_integer_from_twos_complement_limbs_asc(gm: GenMode, config: GenConfig, limit: usize) {
    for xs in unsigned_vec_gen().get(gm, &config).take(limit) {
        println!(
            "from_twos_complement_limbs_asc({:?}) = {:?}",
            xs,
            Integer::from_twos_complement_limbs_asc(&xs)
        );
    }
}

fn demo_integer_from_twos_complement_limbs_desc(gm: GenMode, config: GenConfig, limit: usize) {
    for xs in unsigned_vec_gen().get(gm, &config).take(limit) {
        println!(
            "from_twos_complement_limbs_desc({:?}) = {:?}",
            xs,
            Integer::from_twos_complement_limbs_desc(&xs)
        );
    }
}

fn demo_integer_from_owned_twos_complement_limbs_asc(gm: GenMode, config: GenConfig, limit: usize) {
    for xs in unsigned_vec_gen().get(gm, &config).take(limit) {
        println!(
            "from_owned_twos_complement_limbs_asc({:?}) = {:?}",
            xs.clone(),
            Integer::from_owned_twos_complement_limbs_asc(xs)
        );
    }
}

fn demo_integer_from_owned_twos_complement_limbs_desc(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) {
    for xs in unsigned_vec_gen().get(gm, &config).take(limit) {
        println!(
            "from_owned_twos_complement_limbs_desc({:?}) = {:?}",
            xs.clone(),
            Integer::from_owned_twos_complement_limbs_desc(xs)
        );
    }
}

fn benchmark_integer_from_twos_complement_limbs_asc_evaluation_strategy(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer::from_twos_complement_limbs_asc(&[Limb])",
        BenchmarkType::EvaluationStrategy,
        unsigned_vec_gen().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &vec_len_bucketer(),
        &mut [
            (
                "Integer::from_twos_complement_limbs_asc(&[Limb])",
                &mut |xs| no_out!(Integer::from_twos_complement_limbs_asc(&xs)),
            ),
            (
                "Integer::from_owned_twos_complement_limbs_asc(Vec<Limb>)",
                &mut |xs| no_out!(Integer::from_owned_twos_complement_limbs_asc(xs)),
            ),
        ],
    );
}

fn benchmark_integer_from_twos_complement_limbs_desc_evaluation_strategy(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer::from_twos_complement_limbs_desc(&[Limb])",
        BenchmarkType::EvaluationStrategy,
        unsigned_vec_gen().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &vec_len_bucketer(),
        &mut [
            (
                "Integer::from_twos_complement_limbs_desc(&[Limb])",
                &mut |xs| no_out!(Integer::from_twos_complement_limbs_desc(&xs)),
            ),
            (
                "Integer::from_owned_twos_complement_limbs_desc(Vec<Limb>)",
                &mut |xs| no_out!(Integer::from_owned_twos_complement_limbs_desc(xs)),
            ),
        ],
    );
}
