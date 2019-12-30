use malachite_base::num::arithmetic::traits::DivisibleBy;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_nz::integer::Integer;
use num::{BigInt, Integer as NumInteger, Zero as NumZero};

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::integer::{nrm_pairs_of_integers, pairs_of_integers};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_integer_divisible_by);
    register_demo!(registry, demo_integer_divisible_by_val_ref);
    register_demo!(registry, demo_integer_divisible_by_ref_val);
    register_demo!(registry, demo_integer_divisible_by_ref_ref);
    register_bench!(registry, Large, benchmark_integer_divisible_by_algorithms);
    register_bench!(
        registry,
        Large,
        benchmark_integer_divisible_by_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_divisible_by_library_comparison
    );
}

fn num_divisible_by(x: BigInt, y: BigInt) -> bool {
    x == BigInt::zero() || y != BigInt::zero() && x.is_multiple_of(&y)
}

fn demo_integer_divisible_by(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_integers(gm).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        if x.divisible_by(y) {
            println!("{} is divisible by {}", x_old, y_old);
        } else {
            println!("{} is not divisible by {}", x_old, y_old);
        }
    }
}

fn demo_integer_divisible_by_val_ref(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_integers(gm).take(limit) {
        let x_old = x.clone();
        if x.divisible_by(&y) {
            println!("{} is divisible by {}", x_old, y);
        } else {
            println!("{} is not divisible by {}", x_old, y);
        }
    }
}

fn demo_integer_divisible_by_ref_val(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_integers(gm).take(limit) {
        let y_old = y.clone();
        if (&x).divisible_by(y) {
            println!("{} is divisible by {}", x, y_old);
        } else {
            println!("{} is not divisible by {}", x, y_old);
        }
    }
}

fn demo_integer_divisible_by_ref_ref(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_integers(gm).take(limit) {
        if (&x).divisible_by(&y) {
            println!("{} is divisible by {}", x, y);
        } else {
            println!("{} is not divisible by {}", x, y);
        }
    }
}

fn benchmark_integer_divisible_by_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer.divisible_by(Integer)",
        BenchmarkType::Algorithms,
        pairs_of_integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref x, _)| usize::exact_from(x.significant_bits())),
        "x.significant_bits()",
        &mut [
            ("standard", &mut (|(x, y)| no_out!(x.divisible_by(y)))),
            (
                "using %",
                &mut (|(x, y)| {
                    no_out!(x == Integer::ZERO || y != Integer::ZERO && x % y == Integer::ZERO)
                }),
            ),
        ],
    );
}

fn benchmark_integer_divisible_by_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.divisible_by(Integer)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref x, _)| usize::exact_from(x.significant_bits())),
        "x.significant_bits()",
        &mut [
            (
                "Integer.divisible_by(Integer)",
                &mut (|(x, y)| no_out!(x.divisible_by(y))),
            ),
            (
                "Integer.divisible_by(&Integer)",
                &mut (|(x, y)| no_out!(x.divisible_by(&y))),
            ),
            (
                "(&Integer).divisible_by(Integer)",
                &mut (|(x, y)| no_out!((&x).divisible_by(y))),
            ),
            (
                "(&Integer).divisible_by(&Integer)",
                &mut (|(x, y)| no_out!((&x).divisible_by(&y))),
            ),
        ],
    );
}

fn benchmark_integer_divisible_by_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.divisible_by(Integer)",
        BenchmarkType::LibraryComparison,
        nrm_pairs_of_integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, (ref x, _))| usize::exact_from(x.significant_bits())),
        "y.significant_bits()",
        &mut [
            (
                "malachite",
                &mut (|(_, _, (x, y))| no_out!(x.divisible_by(y))),
            ),
            (
                "num",
                &mut (|((x, y), _, _)| no_out!(num_divisible_by(x, y))),
            ),
            ("rug", &mut (|(_, (x, y), _)| no_out!(x.is_divisible(&y)))),
        ],
    );
}