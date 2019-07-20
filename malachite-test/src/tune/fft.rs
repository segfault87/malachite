use malachite_nz::natural::arithmetic::mul::fft::_limbs_mul_greater_to_out_fft;
use malachite_nz::natural::arithmetic::mul::toom::{
    _limbs_mul_greater_to_out_toom_8h, _limbs_mul_greater_to_out_toom_8h_scratch_size,
};
use malachite_nz::platform::Limb;

use common::GenerationMode;
use inputs::base::triples_of_unsigned_vec_var_34;
use tune::compare_two::{compare_two, ComparisonResult};

pub fn tune() -> Vec<String> {
    let result = compare_two(
        &mut (|(mut out, xs, ys): (Vec<Limb>, Vec<Limb>, Vec<Limb>)| {
            let mut scratch =
                vec![0; _limbs_mul_greater_to_out_toom_8h_scratch_size(xs.len(), ys.len())];
            _limbs_mul_greater_to_out_toom_8h(&mut out, &xs, &ys, &mut scratch)
        }),
        &mut (|(mut out, xs, ys): (Vec<Limb>, Vec<Limb>, Vec<Limb>)| {
            _limbs_mul_greater_to_out_fft(&mut out, &xs, &ys)
        }),
        triples_of_unsigned_vec_var_34(GenerationMode::Random(8192)),
        10000,
        &(|&(_, ref xs, _)| xs.len()),
    );
    let mut lines = Vec::new();
    if let ComparisonResult::SecondBetterAbove(threshold) = result {
        lines.push(format!(
            "pub const MUL_FFT_THRESHOLD: usize = {};",
            threshold
        ));
    } else {
        panic!("Unexpected FFT tuning result: {:?}", result);
    }
    lines
}