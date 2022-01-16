use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base_test_util::generators::{signed_pair_gen, unsigned_pair_gen_var_27};
use malachite_q::Rational;
use malachite_q_test_util::common::rational_to_rug_rational;
use malachite_q_test_util::generators::{rational_signed_pair_gen, rational_unsigned_pair_gen};
use rug;
use std::str::FromStr;

#[test]
fn test_partial_eq_u32() {
    let test = |s, v: u32, out| {
        let u = Rational::from_str(s).unwrap();

        assert_eq!(u == v, out);
        assert_eq!(rug::Rational::from_str(s).unwrap() == v, out);

        assert_eq!(v == u, out);
        assert_eq!(v == rug::Rational::from_str(s).unwrap(), out);
    };
    test("0", 0, true);
    test("0", 5, false);
    test("123", 123, true);
    test("-123", 123, false);
    test("123", 5, false);
    test("-123", 5, false);
    test("1000000000000", 123, false);
    test("-1000000000000", 123, false);
    test("22/7", 123, false);
    test("-22/7", 123, false);
}

#[test]
fn test_partial_eq_u64() {
    let test = |s, v: u64, out| {
        let u = Rational::from_str(s).unwrap();

        assert_eq!(u == v, out);
        assert_eq!(rug::Rational::from_str(s).unwrap() == v, out);

        assert_eq!(v == u, out);
        assert_eq!(v == rug::Rational::from_str(s).unwrap(), out);
    };
    test("0", 0, true);
    test("0", 5, false);
    test("123", 123, true);
    test("-123", 123, false);
    test("123", 5, false);
    test("-123", 5, false);
    test("1000000000000", 1000000000000, true);
    test("-1000000000000", 1000000000000, false);
    test("1000000000000", 1000000000001, false);
    test("-1000000000000", 1000000000001, false);
    test("1000000000000000000000000", 1000000000000, false);
    test("-1000000000000000000000000", 1000000000000, false);
    test("22/7", 123, false);
    test("-22/7", 123, false);
}

#[test]
fn test_partial_eq_i32() {
    let test = |s, v: i32, out| {
        let u = Rational::from_str(s).unwrap();

        assert_eq!(u == v, out);
        assert_eq!(rug::Rational::from_str(s).unwrap() == v, out);

        assert_eq!(v == u, out);
        assert_eq!(v == rug::Rational::from_str(s).unwrap(), out);
    };
    test("0", 0, true);
    test("0", 5, false);
    test("123", 123, true);
    test("-123", -123, true);
    test("-123", 123, false);
    test("123", 5, false);
    test("-123", -5, false);
    test("1000000000000", 123, false);
    test("-1000000000000", -123, false);
    test("22/7", 123, false);
    test("22/7", -123, false);
    test("-22/7", 123, false);
    test("-22/7", -123, false);
}

#[test]
fn test_partial_eq_i64() {
    let test = |s, v: i64, out| {
        let u = Rational::from_str(s).unwrap();

        assert_eq!(u == v, out);
        assert_eq!(rug::Rational::from_str(s).unwrap() == v, out);

        assert_eq!(v == u, out);
        assert_eq!(v == rug::Rational::from_str(s).unwrap(), out);
    };
    test("0", 0, true);
    test("0", 5, false);
    test("123", 123, true);
    test("-123", -123, true);
    test("-123", 123, false);
    test("123", 5, false);
    test("-123", -5, false);
    test("1000000000000", 1000000000000, true);
    test("-1000000000000", -1000000000000, true);
    test("1000000000000", 1000000000001, false);
    test("-1000000000000", -1000000000001, false);
    test("1000000000000000000000000", 1000000000000, false);
    test("-1000000000000000000000000", -1000000000000, false);
    test("22/7", 123, false);
    test("22/7", -123, false);
    test("-22/7", 123, false);
    test("-22/7", -123, false);
}

#[allow(clippy::cmp_owned, clippy::op_ref)] // Extra refs necessary for type inference
fn partial_eq_primitive_int_properties_helper_unsigned<
    T: PartialEq<Rational> + PartialEq<rug::Rational> + PrimitiveUnsigned,
>()
where
    Rational: From<T> + PartialEq<T>,
    rug::Rational: PartialEq<T>,
{
    rational_unsigned_pair_gen::<T>().test_properties(|(n, u)| {
        let eq = n == u;
        assert_eq!(rational_to_rug_rational(&n) == u, eq);
        assert_eq!(&n == &Rational::from(u), eq);

        assert_eq!(u == n, eq);
        assert_eq!(u == rational_to_rug_rational(&n), eq);
        assert_eq!(&Rational::from(u) == &n, eq);
    });

    unsigned_pair_gen_var_27::<T>().test_properties(|(x, y)| {
        assert_eq!(Rational::from(x) == y, x == y);
        assert_eq!(x == Rational::from(y), x == y);
    });
}

#[allow(clippy::cmp_owned, clippy::op_ref)] // Extra refs necessary for type inference
fn partial_eq_primitive_int_properties_helper_signed<
    T: PartialEq<Rational> + PartialEq<rug::Rational> + PrimitiveSigned,
>()
where
    Rational: From<T> + PartialEq<T>,
    rug::Rational: PartialEq<T>,
{
    rational_signed_pair_gen::<T>().test_properties(|(n, i)| {
        let eq = n == i;
        assert_eq!(rational_to_rug_rational(&n) == i, eq);
        assert_eq!(&n == &Rational::from(i), eq);

        assert_eq!(i == n, eq);
        assert_eq!(i == rational_to_rug_rational(&n), eq);
        assert_eq!(&Rational::from(i) == &n, eq);
    });

    signed_pair_gen::<T>().test_properties(|(x, y)| {
        assert_eq!(Rational::from(x) == y, x == y);
        assert_eq!(x == Rational::from(y), x == y);
    });
}

#[test]
fn partial_eq_primitive_int_properties() {
    apply_fn_to_unsigneds!(partial_eq_primitive_int_properties_helper_unsigned);
    apply_fn_to_signeds!(partial_eq_primitive_int_properties_helper_signed);
}
