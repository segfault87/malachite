use malachite_base::limbs::limbs_move_left;
use malachite_nz::platform::Limb;

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::pairs_of_unsigned_vec_and_small_usize_var_1;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_move_left);
    register_bench!(registry, Small, benchmark_limbs_move_left_algorithms);
}

pub fn limbs_move_left_naive<T: Copy>(limbs: &mut [T], amount: usize) {
    let slice = limbs[amount..].to_vec();
    let limit = limbs.len() - amount;
    limbs[..limit].copy_from_slice(&slice);
}

fn demo_limbs_move_left(gm: GenerationMode, limit: usize) {
    for (limbs, amount) in pairs_of_unsigned_vec_and_small_usize_var_1::<Limb>(gm).take(limit) {
        let mut mut_limbs = limbs.clone();
        limbs_move_left(&mut mut_limbs, amount);
        println!(
            "limbs := {:?}; limbs_move_left(&mut limbs, {}); limbs = {:?}",
            limbs, amount, mut_limbs
        );
    }
}

fn benchmark_limbs_move_left_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_move_left(&mut Vec<Limb>, usize)",
        BenchmarkType::Algorithms,
        pairs_of_unsigned_vec_and_small_usize_var_1::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref limbs, _)| limbs.len()),
        "limbs.len()",
        &mut [
            (
                "standard",
                &mut (|(mut limbs, amount)| limbs_move_left(&mut limbs, amount)),
            ),
            (
                "naive",
                &mut (|(mut limbs, amount)| limbs_move_left_naive(&mut limbs, amount)),
            ),
        ],
    );
}