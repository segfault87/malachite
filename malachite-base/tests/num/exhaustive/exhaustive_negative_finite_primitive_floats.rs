use itertools::Itertools;
use malachite_base::num::exhaustive::exhaustive_negative_finite_primitive_floats;
use malachite_base::num::float::nice_float::NiceFloat;
use malachite_base::num::float::PrimitiveFloat;

fn exhaustive_negative_finite_primitive_floats_helper<T: PrimitiveFloat>(out: &[T]) {
    assert_eq!(
        exhaustive_negative_finite_primitive_floats::<T>()
            .take(50)
            .map(NiceFloat)
            .collect_vec(),
        out.iter().copied().map(NiceFloat).collect_vec()
    );
}

#[test]
fn test_exhaustive_negative_finite_primitive_floats() {
    exhaustive_negative_finite_primitive_floats_helper::<f32>(&[
        -1.0, -2.0, -1.5, -0.5, -1.25, -3.0, -1.75, -4.0, -1.125, -2.5, -1.375, -0.75, -1.625,
        -3.5, -1.875, -0.25, -1.0625, -2.25, -1.1875, -0.625, -1.3125, -2.75, -1.4375, -6.0,
        -1.5625, -3.25, -1.6875, -0.875, -1.8125, -3.75, -1.9375, -8.0, -1.03125, -2.125, -1.09375,
        -0.5625, -1.15625, -2.375, -1.21875, -5.0, -1.28125, -2.625, -1.34375, -0.6875, -1.40625,
        -2.875, -1.46875, -0.375, -1.53125, -3.125,
    ]);
    exhaustive_negative_finite_primitive_floats_helper::<f64>(&[
        -1.0, -2.0, -1.5, -0.5, -1.25, -3.0, -1.75, -4.0, -1.125, -2.5, -1.375, -0.75, -1.625,
        -3.5, -1.875, -0.25, -1.0625, -2.25, -1.1875, -0.625, -1.3125, -2.75, -1.4375, -6.0,
        -1.5625, -3.25, -1.6875, -0.875, -1.8125, -3.75, -1.9375, -8.0, -1.03125, -2.125, -1.09375,
        -0.5625, -1.15625, -2.375, -1.21875, -5.0, -1.28125, -2.625, -1.34375, -0.6875, -1.40625,
        -2.875, -1.46875, -0.375, -1.53125, -3.125,
    ]);
}