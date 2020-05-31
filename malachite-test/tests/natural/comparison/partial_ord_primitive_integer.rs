use std::cmp::Ordering;
use std::str::FromStr;

use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_nz::natural::Natural;
use malachite_nz_test_util::common::{natural_to_biguint, natural_to_rug_integer};
use num::BigUint;
use rand::Rand;
use rug;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{pairs_of_natural_signeds, pairs_of_unsigneds};
use malachite_test::inputs::natural::{
    pairs_of_natural_and_natural_signed, pairs_of_natural_and_signed,
    pairs_of_natural_and_unsigned, triples_of_natural_signed_and_natural,
    triples_of_natural_unsigned_and_natural, triples_of_signed_natural_and_signed,
    triples_of_unsigned_natural_and_unsigned,
};
use malachite_test::natural::comparison::partial_ord_primitive_integer::num_partial_cmp_unsigned;

#[test]
fn test_partial_cmp_u32() {
    let test = |u, v: u32, out| {
        assert_eq!(Natural::from_str(u).unwrap().partial_cmp(&v), out);
        assert_eq!(
            num_partial_cmp_unsigned(&BigUint::from_str(u).unwrap(), v),
            out
        );
        assert_eq!(rug::Integer::from_str(u).unwrap().partial_cmp(&v), out);

        assert_eq!(
            v.partial_cmp(&Natural::from_str(u).unwrap()),
            out.map(|o| o.reverse())
        );
    };
    test("0", 0, Some(Ordering::Equal));
    test("0", 5, Some(Ordering::Less));
    test("123", 123, Some(Ordering::Equal));
    test("123", 124, Some(Ordering::Less));
    test("123", 122, Some(Ordering::Greater));
    test("1000000000000", 123, Some(Ordering::Greater));
}

#[test]
fn test_partial_cmp_u64() {
    let test = |u, v: u64, out| {
        assert_eq!(Natural::from_str(u).unwrap().partial_cmp(&v), out);
        assert_eq!(
            num_partial_cmp_unsigned(&BigUint::from_str(u).unwrap(), v),
            out
        );
        assert_eq!(rug::Integer::from_str(u).unwrap().partial_cmp(&v), out);

        assert_eq!(
            v.partial_cmp(&Natural::from_str(u).unwrap()),
            out.map(|o| o.reverse())
        );
    };
    test("0", 0, Some(Ordering::Equal));
    test("0", 5, Some(Ordering::Less));
    test("123", 123, Some(Ordering::Equal));
    test("123", 124, Some(Ordering::Less));
    test("123", 122, Some(Ordering::Greater));
    test("1000000000000", 123, Some(Ordering::Greater));
    test("1000000000000", 1000000000000, Some(Ordering::Equal));
    test("1000000000000", 1000000000001, Some(Ordering::Less));
}

#[test]
fn test_partial_cmp_i32() {
    let test = |u, v: i32, out| {
        assert_eq!(Natural::from_str(u).unwrap().partial_cmp(&v), out);
        assert_eq!(rug::Integer::from_str(u).unwrap().partial_cmp(&v), out);

        assert_eq!(
            v.partial_cmp(&Natural::from_str(u).unwrap()),
            out.map(|o| o.reverse())
        );
    };
    test("0", 0, Some(Ordering::Equal));
    test("0", 5, Some(Ordering::Less));
    test("0", -5, Some(Ordering::Greater));
    test("123", 123, Some(Ordering::Equal));
    test("123", -123, Some(Ordering::Greater));
    test("123", 124, Some(Ordering::Less));
    test("123", -124, Some(Ordering::Greater));
    test("123", 122, Some(Ordering::Greater));
    test("123", -122, Some(Ordering::Greater));
    test("1000000000000", 123, Some(Ordering::Greater));
    test("1000000000000", -123, Some(Ordering::Greater));
}

#[test]
fn test_partial_cmp_i64() {
    let test = |u, v: i64, out| {
        assert_eq!(Natural::from_str(u).unwrap().partial_cmp(&v), out);
        assert_eq!(rug::Integer::from_str(u).unwrap().partial_cmp(&v), out);

        assert_eq!(
            v.partial_cmp(&Natural::from_str(u).unwrap()),
            out.map(|o| o.reverse())
        );
    };
    test("0", 0, Some(Ordering::Equal));
    test("0", 5, Some(Ordering::Less));
    test("0", -5, Some(Ordering::Greater));
    test("123", 123, Some(Ordering::Equal));
    test("123", -123, Some(Ordering::Greater));
    test("123", 124, Some(Ordering::Less));
    test("123", -124, Some(Ordering::Greater));
    test("123", 122, Some(Ordering::Greater));
    test("123", -122, Some(Ordering::Greater));
    test("1000000000000", 123, Some(Ordering::Greater));
    test("1000000000000", -123, Some(Ordering::Greater));
    test("1000000000000", 1000000000000, Some(Ordering::Equal));
    test("1000000000000", -1000000000000, Some(Ordering::Greater));
    test("1000000000000", 1000000000001, Some(Ordering::Less));
    test("1000000000000", -1000000000001, Some(Ordering::Greater));
}

fn partial_cmp_primitive_integer_properties_helper_unsigned<
    T: PartialOrd<Natural> + PartialOrd<rug::Integer> + PrimitiveUnsigned + Rand,
>()
where
    Natural: From<T> + PartialOrd<T>,
    BigUint: From<T>,
    rug::Integer: PartialOrd<T>,
{
    test_properties(pairs_of_natural_and_unsigned::<T>, |&(ref n, u)| {
        let cmp = n.partial_cmp(&u);
        assert_eq!(num_partial_cmp_unsigned(&natural_to_biguint(n), u), cmp);
        assert_eq!(natural_to_rug_integer(n).partial_cmp(&u), cmp);
        assert_eq!(Some(n.cmp(&Natural::from(u))), cmp);

        let cmp_rev = cmp.map(|o| o.reverse());
        assert_eq!(u.partial_cmp(n), cmp_rev);
        assert_eq!(u.partial_cmp(&natural_to_rug_integer(n)), cmp_rev);
        assert_eq!(Some(Natural::from(u).cmp(n)), cmp_rev);
    });

    test_properties(
        triples_of_natural_unsigned_and_natural::<T>,
        |&(ref n, u, ref m): &(Natural, T, Natural)| {
            if *n < u && u < *m {
                assert_eq!(n.cmp(m), Ordering::Less);
            } else if *n > u && u > *m {
                assert_eq!(n.cmp(m), Ordering::Greater);
            }
        },
    );

    test_properties(
        triples_of_unsigned_natural_and_unsigned::<T>,
        |&(u, ref n, v)| {
            if u < *n && *n < v {
                assert!(u < v);
            } else if u > *n && *n > v {
                assert!(u > v);
            }
        },
    );

    test_properties(pairs_of_unsigneds::<T>, |&(x, y)| {
        assert_eq!(Natural::from(x).partial_cmp(&y), Some(x.cmp(&y)));
        assert_eq!(x.partial_cmp(&Natural::from(y)), Some(x.cmp(&y)));
    });
}

fn partial_cmp_primitive_integer_properties_helper_signed<
    T: PartialOrd<Natural> + PartialOrd<rug::Integer> + PrimitiveSigned + Rand,
>()
where
    Natural: ExactFrom<T> + PartialOrd<T>,
    rug::Integer: PartialOrd<T>,
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    test_properties(pairs_of_natural_and_signed::<T>, |&(ref n, i)| {
        let cmp = n.partial_cmp(&i);
        assert_eq!(natural_to_rug_integer(n).partial_cmp(&i), cmp);

        let cmp_rev = cmp.map(|o| o.reverse());
        assert_eq!(i.partial_cmp(n), cmp_rev);
        assert_eq!(i.partial_cmp(&natural_to_rug_integer(n)), cmp_rev);
    });

    test_properties(pairs_of_natural_and_natural_signed::<T>, |&(ref n, i)| {
        let cmp = n.partial_cmp(&i);
        assert_eq!(Some(n.cmp(&Natural::exact_from(i))), cmp);

        let cmp_rev = cmp.map(|o| o.reverse());
        assert_eq!(Some(Natural::exact_from(i).cmp(n)), cmp_rev);
    });

    test_properties(
        triples_of_natural_signed_and_natural::<T>,
        |&(ref n, i, ref m): &(Natural, T, Natural)| {
            if *n < i && i < *m {
                assert_eq!(n.cmp(m), Ordering::Less);
            } else if *n > i && i > *m {
                assert_eq!(n.cmp(m), Ordering::Greater);
            }
        },
    );

    test_properties(
        triples_of_signed_natural_and_signed::<T>,
        |&(i, ref n, j)| {
            if i < *n && *n < j {
                assert!(i < j);
            } else if i > *n && *n > j {
                assert!(i > j);
            }
        },
    );

    test_properties(pairs_of_natural_signeds::<T>, |&(x, y)| {
        assert_eq!(Natural::exact_from(x).partial_cmp(&y), Some(x.cmp(&y)));
        assert_eq!(x.partial_cmp(&Natural::exact_from(y)), Some(x.cmp(&y)));
    });
}

#[test]
fn partial_cmp_primitive_integer_properties() {
    partial_cmp_primitive_integer_properties_helper_unsigned::<u8>();
    partial_cmp_primitive_integer_properties_helper_unsigned::<u16>();
    partial_cmp_primitive_integer_properties_helper_unsigned::<u32>();
    partial_cmp_primitive_integer_properties_helper_unsigned::<u64>();
    partial_cmp_primitive_integer_properties_helper_unsigned::<usize>();
    partial_cmp_primitive_integer_properties_helper_signed::<i8>();
    partial_cmp_primitive_integer_properties_helper_signed::<i16>();
    partial_cmp_primitive_integer_properties_helper_signed::<i32>();
    partial_cmp_primitive_integer_properties_helper_signed::<i64>();
    partial_cmp_primitive_integer_properties_helper_signed::<isize>();
}
