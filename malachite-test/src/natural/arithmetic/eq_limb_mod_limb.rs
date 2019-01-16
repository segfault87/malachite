use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::triples_of_unsigned_vec_unsigned_and_positive_unsigned_var_1;
#[cfg(feature = "32_bit_limbs")]
use inputs::natural::rm_triples_of_natural_unsigned_and_unsigned;
use inputs::natural::{
    triples_of_natural_unsigned_and_unsigned, triples_of_unsigned_natural_and_unsigned,
};
use malachite_base::num::{EqMod, SignificantBits};
use malachite_nz::natural::arithmetic::eq_limb_mod_limb::{
    _combined_limbs_eq_limb_mod_limb, limbs_eq_limb_mod_limb,
};
use malachite_nz::natural::arithmetic::mod_limb::limbs_mod_limb;
use malachite_nz::platform::Limb;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_eq_limb_mod_limb);
    register_demo!(registry, demo_natural_eq_limb_mod_limb);
    register_demo!(registry, demo_limb_eq_natural_mod_limb);
    register_bench!(registry, Small, benchmark_limbs_eq_limb_mod_limb_algorithms);
    #[cfg(feature = "32_bit_limbs")]
    register_bench!(
        registry,
        Large,
        benchmark_natural_eq_limb_mod_limb_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_eq_limb_mod_limb_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_limb_eq_natural_mod_limb_algorithms
    );
}

fn demo_limbs_eq_limb_mod_limb(gm: GenerationMode, limit: usize) {
    for (limbs, limb, modulus) in
        triples_of_unsigned_vec_unsigned_and_positive_unsigned_var_1(gm).take(limit)
    {
        println!(
            "limbs_eq_limb_mod_limb({:?}, {}, {}) = {}",
            limbs,
            limb,
            modulus,
            limbs_eq_limb_mod_limb(&limbs, limb, modulus)
        );
    }
}

fn demo_natural_eq_limb_mod_limb(gm: GenerationMode, limit: usize) {
    for (n, u, modulus) in triples_of_natural_unsigned_and_unsigned::<Limb>(gm).take(limit) {
        if n.eq_mod(u, modulus) {
            println!("{} is equal to {} mod {}", n, u, modulus);
        } else {
            println!("{} is not equal to {} mod {}", n, u, modulus);
        }
    }
}

fn demo_limb_eq_natural_mod_limb(gm: GenerationMode, limit: usize) {
    for (u, n, modulus) in triples_of_unsigned_natural_and_unsigned::<Limb>(gm).take(limit) {
        if u.eq_mod(&n, modulus) {
            println!("{} is equal to {} mod {}", u, n, modulus);
        } else {
            println!("{} is not equal to {} mod {}", u, n, modulus);
        }
    }
}

fn benchmark_limbs_eq_limb_mod_limb_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    let new_gm = match gm {
        GenerationMode::Random(scale) => GenerationMode::Random(scale * 10),
        GenerationMode::SpecialRandom(scale) => GenerationMode::SpecialRandom(scale * 10),
        gm => gm,
    };
    m_run_benchmark(
        "limbs_eq_limb_mod_limb(&mut [Limb], Limb, Limb)",
        BenchmarkType::Algorithms,
        triples_of_unsigned_vec_unsigned_and_positive_unsigned_var_1(new_gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref limbs, _, _)| limbs.len()),
        "limbs.len()",
        &mut [
            (
                "limbs_eq_limb_mod_limb",
                &mut (|(ref limbs, limb, modulus)| {
                    no_out!(limbs_eq_limb_mod_limb(limbs, limb, modulus))
                }),
            ),
            (
                "limbs_mod_limb",
                &mut (|(ref limbs, limb, modulus)| {
                    no_out!(limbs_mod_limb(limbs, modulus) == limb % modulus)
                }),
            ),
            (
                "_combined_limbs_eq_limb_mod_limb",
                &mut (|(ref limbs, limb, modulus)| {
                    no_out!(_combined_limbs_eq_limb_mod_limb(limbs, limb, modulus))
                }),
            ),
        ],
    );
}

#[cfg(feature = "32_bit_limbs")]
fn benchmark_natural_eq_limb_mod_limb_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.eq_mod(Limb, Limb)",
        BenchmarkType::LibraryComparison,
        rm_triples_of_natural_unsigned_and_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (ref n, _, _))| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "malachite",
                &mut (|(_, (n, u, modulus))| no_out!(n.eq_mod(u, modulus))),
            ),
            (
                "rug",
                &mut (|((n, u, modulus), _)| no_out!(n.is_congruent_u(u, modulus))),
            ),
        ],
    );
}

fn benchmark_natural_eq_limb_mod_limb_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.eq_mod(Limb, Limb)",
        BenchmarkType::Algorithms,
        triples_of_natural_unsigned_and_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "Natural.eq_mod(Limb, Limb)",
                &mut (|(n, u, modulus)| no_out!(n.eq_mod(u, modulus))),
            ),
            (
                "Natural == Limb || Limb != 0 && Natural % Limb == Limb % Limb",
                &mut (|(n, u, modulus)| {
                    no_out!(n == u || modulus != 0 && n % modulus == u % modulus)
                }),
            ),
        ],
    );
}

fn benchmark_limb_eq_natural_mod_limb_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Limb.eq_mod(&Natural, Limb)",
        BenchmarkType::Algorithms,
        triples_of_unsigned_natural_and_unsigned::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "Limb.eq_mod(&Natural, Limb)",
                &mut (|(u, ref n, modulus)| no_out!(u.eq_mod(n, modulus))),
            ),
            (
                "Limb == Natural || Limb != 0 && Limb % Limb == Natural % Limb",
                &mut (|(n, u, modulus)| {
                    no_out!(u == n || modulus != 0 && u % modulus == n % modulus)
                }),
            ),
        ],
    );
}