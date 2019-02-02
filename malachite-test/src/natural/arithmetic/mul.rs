use common::{
    m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, NoSpecialGenerationMode,
    ScaleType,
};
use inputs::base::{
    pairs_of_small_usizes, triples_of_unsigned_vec_var_10, triples_of_unsigned_vec_var_11,
    triples_of_unsigned_vec_var_12, triples_of_unsigned_vec_var_13, triples_of_unsigned_vec_var_14,
    triples_of_unsigned_vec_var_15, triples_of_unsigned_vec_var_16, triples_of_unsigned_vec_var_17,
    triples_of_unsigned_vec_var_18, triples_of_unsigned_vec_var_19, triples_of_unsigned_vec_var_20,
};
use inputs::natural::{nrm_pairs_of_naturals, pairs_of_naturals, rm_pairs_of_naturals};
use malachite_base::num::SignificantBits;
use malachite_nz::natural::arithmetic::mul::toom::{
    _limbs_mul_greater_to_out_toom_22, _limbs_mul_greater_to_out_toom_22_input_sizes_valid,
    _limbs_mul_greater_to_out_toom_22_scratch_size, _limbs_mul_greater_to_out_toom_32,
    _limbs_mul_greater_to_out_toom_32_input_sizes_valid,
    _limbs_mul_greater_to_out_toom_32_scratch_size, _limbs_mul_greater_to_out_toom_33,
    _limbs_mul_greater_to_out_toom_33_input_sizes_valid,
    _limbs_mul_greater_to_out_toom_33_scratch_size, _limbs_mul_greater_to_out_toom_42,
    _limbs_mul_greater_to_out_toom_42_input_sizes_valid,
    _limbs_mul_greater_to_out_toom_42_scratch_size, _limbs_mul_greater_to_out_toom_43,
    _limbs_mul_greater_to_out_toom_43_input_sizes_valid,
    _limbs_mul_greater_to_out_toom_43_scratch_size, _limbs_mul_greater_to_out_toom_44,
    _limbs_mul_greater_to_out_toom_44_input_sizes_valid,
    _limbs_mul_greater_to_out_toom_44_scratch_size, _limbs_mul_greater_to_out_toom_52,
    _limbs_mul_greater_to_out_toom_52_input_sizes_valid,
    _limbs_mul_greater_to_out_toom_52_scratch_size, _limbs_mul_greater_to_out_toom_53,
    _limbs_mul_greater_to_out_toom_53_input_sizes_valid,
    _limbs_mul_greater_to_out_toom_53_scratch_size, _limbs_mul_greater_to_out_toom_54,
    _limbs_mul_greater_to_out_toom_54_input_sizes_valid,
    _limbs_mul_greater_to_out_toom_54_scratch_size, _limbs_mul_greater_to_out_toom_62,
    _limbs_mul_greater_to_out_toom_62_input_sizes_valid,
    _limbs_mul_greater_to_out_toom_62_scratch_size,
};
use malachite_nz::natural::arithmetic::mul::{
    _limbs_mul_greater_to_out_basecase, limbs_mul_greater_to_out,
};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_ns_demo!(
        registry,
        demo_limbs_mul_greater_to_out_toom_22_input_sizes_valid
    );
    register_ns_demo!(
        registry,
        demo_limbs_mul_greater_to_out_toom_32_input_sizes_valid
    );
    register_ns_demo!(
        registry,
        demo_limbs_mul_greater_to_out_toom_33_input_sizes_valid
    );
    register_ns_demo!(
        registry,
        demo_limbs_mul_greater_to_out_toom_42_input_sizes_valid
    );
    register_ns_demo!(
        registry,
        demo_limbs_mul_greater_to_out_toom_43_input_sizes_valid
    );
    register_ns_demo!(
        registry,
        demo_limbs_mul_greater_to_out_toom_44_input_sizes_valid
    );
    register_ns_demo!(
        registry,
        demo_limbs_mul_greater_to_out_toom_52_input_sizes_valid
    );
    register_ns_demo!(
        registry,
        demo_limbs_mul_greater_to_out_toom_53_input_sizes_valid
    );
    register_ns_demo!(
        registry,
        demo_limbs_mul_greater_to_out_toom_54_input_sizes_valid
    );
    register_ns_demo!(
        registry,
        demo_limbs_mul_greater_to_out_toom_62_input_sizes_valid
    );
    register_demo!(registry, demo_natural_mul_assign);
    register_demo!(registry, demo_natural_mul_assign_ref);
    register_demo!(registry, demo_natural_mul);
    register_demo!(registry, demo_natural_mul_val_ref);
    register_demo!(registry, demo_natural_mul_ref_val);
    register_demo!(registry, demo_natural_mul_ref_ref);
    register_bench!(
        registry,
        Large,
        benchmark_limbs_mul_greater_to_out_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_limbs_mul_greater_to_out_toom_22_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_limbs_mul_greater_to_out_toom_32_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_limbs_mul_greater_to_out_toom_33_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_limbs_mul_greater_to_out_toom_42_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_limbs_mul_greater_to_out_toom_43_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_limbs_mul_greater_to_out_toom_44_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_limbs_mul_greater_to_out_toom_52_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_limbs_mul_greater_to_out_toom_53_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_limbs_mul_greater_to_out_toom_54_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_limbs_mul_greater_to_out_toom_62_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_mul_assign_library_comparison
    );
    register_bench!(registry, Large, benchmark_natural_mul_assign_algorithms);
    register_bench!(
        registry,
        Large,
        benchmark_natural_mul_assign_evaluation_strategy
    );
    register_bench!(registry, Large, benchmark_natural_mul_library_comparison);
    register_bench!(registry, Large, benchmark_natural_mul_evaluation_strategy);
}

fn demo_limbs_mul_greater_to_out_toom_22_input_sizes_valid(
    gm: NoSpecialGenerationMode,
    limit: usize,
) {
    for (x, y) in pairs_of_small_usizes(gm).take(limit) {
        println!(
            "_limbs_mul_greater_to_out_toom_22_input_sizes_valid({}, {}) = {}",
            x,
            y,
            _limbs_mul_greater_to_out_toom_22_input_sizes_valid(x, y)
        );
    }
}

fn demo_limbs_mul_greater_to_out_toom_32_input_sizes_valid(
    gm: NoSpecialGenerationMode,
    limit: usize,
) {
    for (x, y) in pairs_of_small_usizes(gm).take(limit) {
        println!(
            "_limbs_mul_greater_to_out_toom_32_input_sizes_valid({}, {}) = {}",
            x,
            y,
            _limbs_mul_greater_to_out_toom_32_input_sizes_valid(x, y)
        );
    }
}

fn demo_limbs_mul_greater_to_out_toom_33_input_sizes_valid(
    gm: NoSpecialGenerationMode,
    limit: usize,
) {
    for (x, y) in pairs_of_small_usizes(gm).take(limit) {
        println!(
            "_limbs_mul_greater_to_out_toom_33_input_sizes_valid({}, {}) = {}",
            x,
            y,
            _limbs_mul_greater_to_out_toom_33_input_sizes_valid(x, y)
        );
    }
}

fn demo_limbs_mul_greater_to_out_toom_42_input_sizes_valid(
    gm: NoSpecialGenerationMode,
    limit: usize,
) {
    for (x, y) in pairs_of_small_usizes(gm).take(limit) {
        println!(
            "_limbs_mul_greater_to_out_toom_42_input_sizes_valid({}, {}) = {}",
            x,
            y,
            _limbs_mul_greater_to_out_toom_42_input_sizes_valid(x, y)
        );
    }
}

fn demo_limbs_mul_greater_to_out_toom_43_input_sizes_valid(
    gm: NoSpecialGenerationMode,
    limit: usize,
) {
    for (x, y) in pairs_of_small_usizes(gm).take(limit) {
        println!(
            "_limbs_mul_greater_to_out_toom_43_input_sizes_valid({}, {}) = {}",
            x,
            y,
            _limbs_mul_greater_to_out_toom_43_input_sizes_valid(x, y)
        );
    }
}

fn demo_limbs_mul_greater_to_out_toom_44_input_sizes_valid(
    gm: NoSpecialGenerationMode,
    limit: usize,
) {
    for (x, y) in pairs_of_small_usizes(gm).take(limit) {
        println!(
            "_limbs_mul_greater_to_out_toom_44_input_sizes_valid({}, {}) = {}",
            x,
            y,
            _limbs_mul_greater_to_out_toom_44_input_sizes_valid(x, y)
        );
    }
}

fn demo_limbs_mul_greater_to_out_toom_52_input_sizes_valid(
    gm: NoSpecialGenerationMode,
    limit: usize,
) {
    for (x, y) in pairs_of_small_usizes(gm).take(limit) {
        println!(
            "_limbs_mul_greater_to_out_toom_52_input_sizes_valid({}, {}) = {}",
            x,
            y,
            _limbs_mul_greater_to_out_toom_52_input_sizes_valid(x, y)
        );
    }
}

fn demo_limbs_mul_greater_to_out_toom_53_input_sizes_valid(
    gm: NoSpecialGenerationMode,
    limit: usize,
) {
    for (x, y) in pairs_of_small_usizes(gm).take(limit) {
        println!(
            "_limbs_mul_greater_to_out_toom_53_input_sizes_valid({}, {}) = {}",
            x,
            y,
            _limbs_mul_greater_to_out_toom_53_input_sizes_valid(x, y)
        );
    }
}

fn demo_limbs_mul_greater_to_out_toom_54_input_sizes_valid(
    gm: NoSpecialGenerationMode,
    limit: usize,
) {
    for (x, y) in pairs_of_small_usizes(gm).take(limit) {
        println!(
            "_limbs_mul_greater_to_out_toom_54_input_sizes_valid({}, {}) = {}",
            x,
            y,
            _limbs_mul_greater_to_out_toom_54_input_sizes_valid(x, y)
        );
    }
}

fn demo_limbs_mul_greater_to_out_toom_62_input_sizes_valid(
    gm: NoSpecialGenerationMode,
    limit: usize,
) {
    for (x, y) in pairs_of_small_usizes(gm).take(limit) {
        println!(
            "_limbs_mul_greater_to_out_toom_62_input_sizes_valid({}, {}) = {}",
            x,
            y,
            _limbs_mul_greater_to_out_toom_62_input_sizes_valid(x, y)
        );
    }
}

fn demo_natural_mul_assign(gm: GenerationMode, limit: usize) {
    for (mut x, y) in pairs_of_naturals(gm).take(limit) {
        let x_old = x.clone();
        x *= y.clone();
        println!("x := {}; x *= {}; x = {}", x_old, y, x);
    }
}

fn demo_natural_mul_assign_ref(gm: GenerationMode, limit: usize) {
    for (mut x, y) in pairs_of_naturals(gm).take(limit) {
        let x_old = x.clone();
        x *= &y;
        println!("x := {}; x *= &{}; x = {}", x_old, y, x);
    }
}

fn demo_natural_mul(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_naturals(gm).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!("{} * {} = {}", x_old, y_old, x * y);
    }
}

fn demo_natural_mul_val_ref(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_naturals(gm).take(limit) {
        let x_old = x.clone();
        println!("{} * &{} = {}", x_old, y, x * &y);
    }
}

fn demo_natural_mul_ref_val(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_naturals(gm).take(limit) {
        let y_old = y.clone();
        println!("&{} * {} = {}", x, y_old, &x * y);
    }
}

fn demo_natural_mul_ref_ref(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_naturals(gm).take(limit) {
        println!("&{} * &{} = {}", x, y, &x * &y);
    }
}

fn benchmark_limbs_mul_greater_to_out_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "limbs_mul_greater_to_out(&mut [u32], &[u32], &[u32])",
        BenchmarkType::Algorithms,
        triples_of_unsigned_vec_var_10(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref xs, ref ys)| xs.len() + ys.len()),
        "x.len() + y.len()",
        &mut [
            (
                "basecase",
                &mut (|(mut out, xs, ys)| _limbs_mul_greater_to_out_basecase(&mut out, &xs, &ys)),
            ),
            (
                "full",
                &mut (|(mut out, xs, ys)| no_out!(limbs_mul_greater_to_out(&mut out, &xs, &ys))),
            ),
        ],
    );
}

fn benchmark_limbs_mul_greater_to_out_toom_22_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "_limbs_mul_greater_to_out_toom_22(&mut [u32], &[u32], &[u32])",
        BenchmarkType::Algorithms,
        triples_of_unsigned_vec_var_11(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref xs, ref ys)| xs.len() + ys.len()),
        "x.len() + y.len()",
        &mut [
            (
                "basecase",
                &mut (|(mut out, xs, ys)| _limbs_mul_greater_to_out_basecase(&mut out, &xs, &ys)),
            ),
            (
                "Toom22",
                &mut (|(mut out, xs, ys)| {
                    let mut scratch =
                        vec![0; _limbs_mul_greater_to_out_toom_22_scratch_size(xs.len())];
                    _limbs_mul_greater_to_out_toom_22(&mut out, &xs, &ys, &mut scratch)
                }),
            ),
        ],
    );
}

fn benchmark_limbs_mul_greater_to_out_toom_32_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "_limbs_mul_greater_to_out_toom_32(&mut [u32], &[u32], &[u32])",
        BenchmarkType::Algorithms,
        triples_of_unsigned_vec_var_12(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref xs, ref ys)| xs.len() + ys.len()),
        "x.len() + y.len()",
        &mut [
            (
                "basecase",
                &mut (|(mut out, xs, ys)| _limbs_mul_greater_to_out_basecase(&mut out, &xs, &ys)),
            ),
            (
                "Toom32",
                &mut (|(mut out, xs, ys)| {
                    let mut scratch =
                        vec![0; _limbs_mul_greater_to_out_toom_32_scratch_size(xs.len(), ys.len())];
                    _limbs_mul_greater_to_out_toom_32(&mut out, &xs, &ys, &mut scratch)
                }),
            ),
        ],
    );
}

fn benchmark_limbs_mul_greater_to_out_toom_33_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "_limbs_mul_greater_to_out_toom_33(&mut [u32], &[u32], &[u32])",
        BenchmarkType::Algorithms,
        triples_of_unsigned_vec_var_13(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref xs, ref ys)| xs.len() + ys.len()),
        "x.len() + y.len()",
        &mut [
            (
                "basecase",
                &mut (|(mut out, xs, ys)| _limbs_mul_greater_to_out_basecase(&mut out, &xs, &ys)),
            ),
            (
                "Toom33",
                &mut (|(mut out, xs, ys)| {
                    let mut scratch =
                        vec![0; _limbs_mul_greater_to_out_toom_33_scratch_size(xs.len())];
                    _limbs_mul_greater_to_out_toom_33(&mut out, &xs, &ys, &mut scratch)
                }),
            ),
        ],
    );
}

fn benchmark_limbs_mul_greater_to_out_toom_42_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "_limbs_mul_greater_to_out_toom_42(&mut [u32], &[u32], &[u32])",
        BenchmarkType::Algorithms,
        triples_of_unsigned_vec_var_14(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref xs, ref ys)| xs.len() + ys.len()),
        "x.len() + y.len()",
        &mut [
            (
                "basecase",
                &mut (|(mut out, xs, ys)| _limbs_mul_greater_to_out_basecase(&mut out, &xs, &ys)),
            ),
            (
                "Toom42",
                &mut (|(mut out, xs, ys)| {
                    let mut scratch =
                        vec![0; _limbs_mul_greater_to_out_toom_42_scratch_size(xs.len(), ys.len())];
                    _limbs_mul_greater_to_out_toom_42(&mut out, &xs, &ys, &mut scratch)
                }),
            ),
        ],
    );
}

fn benchmark_limbs_mul_greater_to_out_toom_43_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "_limbs_mul_greater_to_out_toom_43(&mut [u32], &[u32], &[u32])",
        BenchmarkType::Algorithms,
        triples_of_unsigned_vec_var_15(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref xs, ref ys)| xs.len() + ys.len()),
        "x.len() + y.len()",
        &mut [
            (
                "basecase",
                &mut (|(mut out, xs, ys)| _limbs_mul_greater_to_out_basecase(&mut out, &xs, &ys)),
            ),
            (
                "Toom43",
                &mut (|(mut out, xs, ys)| {
                    let mut scratch =
                        vec![0; _limbs_mul_greater_to_out_toom_43_scratch_size(xs.len(), ys.len())];
                    _limbs_mul_greater_to_out_toom_43(&mut out, &xs, &ys, &mut scratch)
                }),
            ),
        ],
    );
}

fn benchmark_limbs_mul_greater_to_out_toom_44_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "_limbs_mul_greater_to_out_toom_44(&mut [u32], &[u32], &[u32])",
        BenchmarkType::Algorithms,
        triples_of_unsigned_vec_var_16(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref xs, ref ys)| xs.len() + ys.len()),
        "x.len() + y.len()",
        &mut [
            (
                "basecase",
                &mut (|(mut out, xs, ys)| _limbs_mul_greater_to_out_basecase(&mut out, &xs, &ys)),
            ),
            (
                "Toom44",
                &mut (|(mut out, xs, ys)| {
                    let mut scratch =
                        vec![0; _limbs_mul_greater_to_out_toom_44_scratch_size(xs.len())];
                    _limbs_mul_greater_to_out_toom_44(&mut out, &xs, &ys, &mut scratch)
                }),
            ),
        ],
    );
}

fn benchmark_limbs_mul_greater_to_out_toom_52_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "_limbs_mul_greater_to_out_toom_52(&mut [u32], &[u32], &[u32])",
        BenchmarkType::Algorithms,
        triples_of_unsigned_vec_var_17(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref xs, ref ys)| xs.len() + ys.len()),
        "x.len() + y.len()",
        &mut [
            (
                "basecase",
                &mut (|(mut out, xs, ys)| _limbs_mul_greater_to_out_basecase(&mut out, &xs, &ys)),
            ),
            (
                "Toom52",
                &mut (|(mut out, xs, ys)| {
                    let mut scratch =
                        vec![0; _limbs_mul_greater_to_out_toom_52_scratch_size(xs.len(), ys.len())];
                    _limbs_mul_greater_to_out_toom_52(&mut out, &xs, &ys, &mut scratch)
                }),
            ),
        ],
    );
}

fn benchmark_limbs_mul_greater_to_out_toom_53_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "_limbs_mul_greater_to_out_toom_53(&mut [u32], &[u32], &[u32])",
        BenchmarkType::Algorithms,
        triples_of_unsigned_vec_var_18(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref xs, ref ys)| xs.len() + ys.len()),
        "x.len() + y.len()",
        &mut [
            (
                "basecase",
                &mut (|(mut out, xs, ys)| _limbs_mul_greater_to_out_basecase(&mut out, &xs, &ys)),
            ),
            (
                "Toom53",
                &mut (|(mut out, xs, ys)| {
                    let mut scratch =
                        vec![0; _limbs_mul_greater_to_out_toom_53_scratch_size(xs.len(), ys.len())];
                    _limbs_mul_greater_to_out_toom_53(&mut out, &xs, &ys, &mut scratch)
                }),
            ),
        ],
    );
}

fn benchmark_limbs_mul_greater_to_out_toom_54_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "_limbs_mul_greater_to_out_toom_54(&mut [u32], &[u32], &[u32])",
        BenchmarkType::Algorithms,
        triples_of_unsigned_vec_var_19(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref xs, ref ys)| xs.len() + ys.len()),
        "x.len() + y.len()",
        &mut [
            (
                "basecase",
                &mut (|(mut out, xs, ys)| _limbs_mul_greater_to_out_basecase(&mut out, &xs, &ys)),
            ),
            (
                "Toom54",
                &mut (|(mut out, xs, ys)| {
                    let mut scratch =
                        vec![0; _limbs_mul_greater_to_out_toom_54_scratch_size(xs.len(), ys.len())];
                    _limbs_mul_greater_to_out_toom_54(&mut out, &xs, &ys, &mut scratch)
                }),
            ),
        ],
    );
}

fn benchmark_limbs_mul_greater_to_out_toom_62_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "_limbs_mul_greater_to_out_toom_62(&mut [u32], &[u32], &[u32])",
        BenchmarkType::Algorithms,
        triples_of_unsigned_vec_var_20(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref xs, ref ys)| xs.len() + ys.len()),
        "x.len() + y.len()",
        &mut [
            (
                "basecase",
                &mut (|(mut out, xs, ys)| _limbs_mul_greater_to_out_basecase(&mut out, &xs, &ys)),
            ),
            (
                "Toom62",
                &mut (|(mut out, xs, ys)| {
                    let mut scratch =
                        vec![0; _limbs_mul_greater_to_out_toom_62_scratch_size(xs.len(), ys.len())];
                    _limbs_mul_greater_to_out_toom_62(&mut out, &xs, &ys, &mut scratch)
                }),
            ),
        ],
    );
}

fn benchmark_natural_mul_assign_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural *= Natural",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (ref x, ref y))| (x.significant_bits() + y.significant_bits()) as usize),
        "x.significant_bits() + y.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, (mut x, y))| x *= y)),
            ("rug", &mut (|((mut x, y), _)| x *= y)),
        ],
    );
}

fn benchmark_natural_mul_assign_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural *= Natural",
        BenchmarkType::Algorithms,
        pairs_of_naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref x, ref y)| (x.significant_bits() + y.significant_bits()) as usize),
        "x.significant_bits() + y.significant_bits()",
        &mut [
            ("basecase", &mut (|(mut x, y)| no_out!(x *= y))),
            (
                "basecase memory-optimized",
                &mut (|(mut x, y)| no_out!(x._mul_assign_basecase_mem_opt(y))),
            ),
        ],
    );
}

fn benchmark_natural_mul_assign_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural *= Natural",
        BenchmarkType::EvaluationStrategy,
        pairs_of_naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref x, ref y)| (x.significant_bits() + y.significant_bits()) as usize),
        "x.significant_bits() + y.significant_bits()",
        &mut [
            ("Natural *= Natural", &mut (|(mut x, y)| no_out!(x *= y))),
            ("Natural *= &Natural", &mut (|(mut x, y)| no_out!(x *= &y))),
        ],
    );
}

fn benchmark_natural_mul_library_comparison(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural * Natural",
        BenchmarkType::LibraryComparison,
        nrm_pairs_of_naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, (ref x, ref y))| (x.significant_bits() + y.significant_bits()) as usize),
        "x.significant_bits() + y.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, _, (x, y))| no_out!(x * y))),
            ("num", &mut (|((x, y), _, _)| no_out!(x * y))),
            ("rug", &mut (|(_, (x, y), _)| no_out!(x * y))),
        ],
    );
}

fn benchmark_natural_mul_evaluation_strategy(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural * Natural",
        BenchmarkType::EvaluationStrategy,
        pairs_of_naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref x, ref y)| (x.significant_bits() + y.significant_bits()) as usize),
        "x.significant_bits() + y.significant_bits()",
        &mut [
            ("Natural * Natural", &mut (|(x, y)| no_out!(x * y))),
            ("Natural * &Natural", &mut (|(x, y)| no_out!(x * &y))),
            ("&Natural * Natural", &mut (|(x, y)| no_out!(&x * y))),
            ("&Natural * &Natural", &mut (|(x, y)| no_out!(&x * &y))),
        ],
    );
}
