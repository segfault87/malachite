use malachite_base::conversion::{CheckedFrom, RoundingFrom};
use malachite_base::named::Named;
use malachite_base::num::traits::SignificantBits;

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::natural::{
    naturals, pairs_of_natural_and_rounding_mode_var_1_f32,
    pairs_of_natural_and_rounding_mode_var_1_f64,
};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_f32_rounding_from_natural);
    register_demo!(registry, demo_f64_rounding_from_natural);
    register_demo!(registry, demo_f32_rounding_from_natural_ref);
    register_demo!(registry, demo_f64_rounding_from_natural_ref);
    register_demo!(registry, demo_f32_from_natural);
    register_demo!(registry, demo_f64_from_natural);
    register_demo!(registry, demo_f32_from_natural_ref);
    register_demo!(registry, demo_f64_from_natural_ref);
    register_demo!(registry, demo_f32_checked_from_natural);
    register_demo!(registry, demo_f64_checked_from_natural);
    register_demo!(registry, demo_f32_checked_from_natural_ref);
    register_demo!(registry, demo_f64_checked_from_natural_ref);
    register_bench!(registry, Small, benchmark_f32_rounding_from_natural);
    register_bench!(registry, Small, benchmark_f64_rounding_from_natural);
    register_bench!(registry, Small, benchmark_f32_from_natural);
    register_bench!(registry, Small, benchmark_f64_from_natural);
    register_bench!(registry, Small, benchmark_f32_checked_from_natural);
    register_bench!(registry, Small, benchmark_f64_checked_from_natural);
}

macro_rules! float_demos_and_benches {
    (
        $f: ident,
        $pairs_of_natural_and_rounding_mode_var_1: ident,
        $demo_float_rounding_from_natural: ident,
        $demo_float_rounding_from_natural_ref: ident,
        $demo_float_from_natural: ident,
        $demo_float_from_natural_ref: ident,
        $demo_float_checked_from_natural: ident,
        $demo_float_checked_from_natural_ref: ident,
        $benchmark_float_rounding_from_natural: ident,
        $benchmark_float_from_natural: ident,
        $benchmark_float_checked_from_natural: ident,
    ) => {
        fn $demo_float_rounding_from_natural(gm: GenerationMode, limit: usize) {
            for (n, rm) in $pairs_of_natural_and_rounding_mode_var_1(gm).take(limit) {
                println!(
                    "{}::rounding_from({}, {}) = {:?}",
                    $f::NAME,
                    n.clone(),
                    rm,
                    $f::rounding_from(n, rm)
                );
            }
        }

        fn $demo_float_rounding_from_natural_ref(gm: GenerationMode, limit: usize) {
            for (n, rm) in $pairs_of_natural_and_rounding_mode_var_1(gm).take(limit) {
                println!(
                    "{}::rounding_from(&{}, {}) = {:?}",
                    $f::NAME,
                    n,
                    rm,
                    $f::rounding_from(&n, rm)
                );
            }
        }

        fn $demo_float_from_natural(gm: GenerationMode, limit: usize) {
            for n in naturals(gm).take(limit) {
                println!("{}::from({}) = {:?}", $f::NAME, n.clone(), $f::from(n));
            }
        }

        fn $demo_float_from_natural_ref(gm: GenerationMode, limit: usize) {
            for n in naturals(gm).take(limit) {
                println!("{}::from({}) = {:?}", $f::NAME, n.clone(), $f::from(n));
            }
        }

        fn $demo_float_checked_from_natural(gm: GenerationMode, limit: usize) {
            for n in naturals(gm).take(limit) {
                println!(
                    "{}::checked_from({}) = {:?}",
                    $f::NAME,
                    n.clone(),
                    $f::checked_from(n)
                );
            }
        }

        fn $demo_float_checked_from_natural_ref(gm: GenerationMode, limit: usize) {
            for n in naturals(gm).take(limit) {
                println!(
                    "{}::checked_from({}) = {:?}",
                    $f::NAME,
                    n.clone(),
                    $f::checked_from(n)
                );
            }
        }

        fn $benchmark_float_rounding_from_natural(
            gm: GenerationMode,
            limit: usize,
            file_name: &str,
        ) {
            m_run_benchmark(
                &format!("{}::rounding_from(Natural, RoundingMode)", stringify!($f)),
                BenchmarkType::EvaluationStrategy,
                $pairs_of_natural_and_rounding_mode_var_1(gm),
                gm.name(),
                limit,
                file_name,
                &(|&(ref n, _)| n.significant_bits() as usize),
                "n.significant_bits()",
                &mut [
                    (
                        &format!("{}::rounding_from(Natural, RoundingMode)", stringify!($f)),
                        &mut (|(n, rm)| no_out!($f::rounding_from(n, rm))),
                    ),
                    (
                        &format!("{}::rounding_from(&Natural, RoundingMode)", stringify!($f)),
                        &mut (|(n, rm)| no_out!($f::rounding_from(&n, rm))),
                    ),
                ],
            );
        }

        fn $benchmark_float_from_natural(gm: GenerationMode, limit: usize, file_name: &str) {
            m_run_benchmark(
                &format!("{}::from(Natural)", stringify!($f)),
                BenchmarkType::EvaluationStrategy,
                naturals(gm),
                gm.name(),
                limit,
                file_name,
                &(|ref n| n.significant_bits() as usize),
                "n.significant_bits()",
                &mut [
                    (
                        &format!("{}::from(Natural)", stringify!($f)),
                        &mut (|n| no_out!($f::from(n))),
                    ),
                    (
                        &format!("{}::from(&Natural)", stringify!($f)),
                        &mut (|n| no_out!($f::from(&n))),
                    ),
                ],
            );
        }

        fn $benchmark_float_checked_from_natural(
            gm: GenerationMode,
            limit: usize,
            file_name: &str,
        ) {
            m_run_benchmark(
                &format!("{}::checked_from(Natural)", stringify!($f)),
                BenchmarkType::EvaluationStrategy,
                naturals(gm),
                gm.name(),
                limit,
                file_name,
                &(|ref n| n.significant_bits() as usize),
                "n.significant_bits()",
                &mut [
                    (
                        &format!("{}::checked_from(Natural)", stringify!($f)),
                        &mut (|n| no_out!($f::checked_from(n))),
                    ),
                    (
                        &format!("{}::checked_from(&Natural)", stringify!($f)),
                        &mut (|n| no_out!($f::checked_from(&n))),
                    ),
                ],
            );
        }
    };
}

float_demos_and_benches!(
    f32,
    pairs_of_natural_and_rounding_mode_var_1_f32,
    demo_f32_rounding_from_natural,
    demo_f32_rounding_from_natural_ref,
    demo_f32_from_natural,
    demo_f32_from_natural_ref,
    demo_f32_checked_from_natural,
    demo_f32_checked_from_natural_ref,
    benchmark_f32_rounding_from_natural,
    benchmark_f32_from_natural,
    benchmark_f32_checked_from_natural,
);

float_demos_and_benches!(
    f64,
    pairs_of_natural_and_rounding_mode_var_1_f64,
    demo_f64_rounding_from_natural,
    demo_f64_rounding_from_natural_ref,
    demo_f64_from_natural,
    demo_f64_from_natural_ref,
    demo_f64_checked_from_natural,
    demo_f64_checked_from_natural_ref,
    benchmark_f64_rounding_from_natural,
    benchmark_f64_from_natural,
    benchmark_f64_checked_from_natural,
);
