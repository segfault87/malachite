use malachite_base::num::arithmetic::traits::Abs;
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::comparison::traits::{OrdAbs, PartialOrdAbs};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_q::conversion::from_primitive_float::RationalFromPrimitiveFloatError;
use malachite_q::test_util::generators::{
    rational_gen, rational_primitive_float_pair_gen,
    rational_primitive_float_primitive_float_triple_gen,
    rational_rational_primitive_float_triple_gen,
};
use malachite_q::Rational;
use std::cmp::Ordering;
use std::str::FromStr;

#[test]
fn test_partial_cmp_abs_primitive_float() {
    let test = |u, v: f32, out: Option<Ordering>| {
        let out_rev = out.map(Ordering::reverse);
        assert_eq!(Rational::from_str(u).unwrap().partial_cmp_abs(&v), out);
        assert_eq!(v.partial_cmp_abs(&Rational::from_str(u).unwrap()), out_rev);

        let v = f64::from(v);
        assert_eq!(Rational::from_str(u).unwrap().partial_cmp_abs(&v), out);
        assert_eq!(v.partial_cmp_abs(&Rational::from_str(u).unwrap()), out_rev);
    };
    test("2/3", f32::NAN, None);
    test("2/3", f32::POSITIVE_INFINITY, Some(Ordering::Less));
    test("2/3", f32::NEGATIVE_INFINITY, Some(Ordering::Less));
    test("-2/3", f32::NAN, None);
    test("-2/3", f32::POSITIVE_INFINITY, Some(Ordering::Less));
    test("-2/3", f32::NEGATIVE_INFINITY, Some(Ordering::Less));

    test("0", 0.0, Some(Ordering::Equal));
    test("0", -0.0, Some(Ordering::Equal));
    test("0", 5.0, Some(Ordering::Less));
    test("0", -5.0, Some(Ordering::Less));
    test("3/2", 1.5, Some(Ordering::Equal));
    test("3/2", 5.0, Some(Ordering::Less));
    test("3/2", -5.0, Some(Ordering::Less));
    test("-3/2", 5.0, Some(Ordering::Less));
    test("-3/2", -5.0, Some(Ordering::Less));
    test("-3/2", -1.5, Some(Ordering::Equal));

    test("1/3", 0.333, Some(Ordering::Greater));
    test("1/3", 0.334, Some(Ordering::Less));
    test("1/3", -0.333, Some(Ordering::Greater));
    test("1/3", -0.334, Some(Ordering::Less));
    test("-1/3", -0.334, Some(Ordering::Less));
    test("-1/3", -0.333, Some(Ordering::Greater));
    test("-1/3", 0.334, Some(Ordering::Less));
    test("-1/3", 0.333, Some(Ordering::Greater));
}

fn partial_cmp_abs_primitive_float_properties_helper<T: PartialOrdAbs<Rational> + PrimitiveFloat>()
where
    Rational:
        TryFrom<T, Error = RationalFromPrimitiveFloatError> + PartialOrd<T> + PartialOrdAbs<T>,
{
    rational_primitive_float_pair_gen::<T>().test_properties(|(n, u)| {
        let cmp_abs = n.partial_cmp_abs(&u);
        let cmp_abs_rev = cmp_abs.map(Ordering::reverse);
        assert_eq!(u.partial_cmp_abs(&n), cmp_abs_rev);

        assert_eq!((&n).abs().partial_cmp(&u.abs()), cmp_abs);

        if u.is_finite() {
            assert_eq!(n.cmp_abs(&Rational::exact_from(u)), cmp_abs.unwrap());
        }
    });

    rational_rational_primitive_float_triple_gen::<T>().test_properties(|(n, m, u)| {
        if n.lt_abs(&u) && u.lt_abs(&m) {
            assert_eq!(n.cmp_abs(&m), Ordering::Less);
        } else if n.gt_abs(&u) && u.gt_abs(&m) {
            assert_eq!(n.cmp_abs(&m), Ordering::Greater);
        }
    });

    rational_primitive_float_primitive_float_triple_gen::<T>().test_properties(|(n, u, v)| {
        if u.lt_abs(&n) && n.lt_abs(&v) {
            assert!(u.abs() < v.abs());
        } else if u.gt_abs(&n) && n.gt_abs(&v) {
            assert!(u.abs() > v.abs());
        }
    });

    rational_gen().test_properties(|x| {
        assert!(x.ge_abs(&T::ZERO));
        assert!(x.lt_abs(&T::NEGATIVE_INFINITY));
        assert!(x.lt_abs(&T::POSITIVE_INFINITY));
    });
}

#[test]
fn partial_cmp_abs_primitive_float_properties() {
    apply_fn_to_primitive_floats!(partial_cmp_abs_primitive_float_properties_helper);
}
