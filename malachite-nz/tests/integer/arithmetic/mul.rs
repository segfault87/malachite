use malachite_base::num::arithmetic::traits::{DivMod, Square};
use malachite_base::num::basic::traits::{NegativeOne, One, Zero};
use malachite_base_test_util::generators::signed_pair_gen;
use malachite_nz::integer::Integer;
use malachite_nz::platform::{SignedDoubleLimb, SignedLimb};
use malachite_nz_test_util::common::{
    bigint_to_integer, integer_to_bigint, integer_to_rug_integer, rug_integer_to_integer,
};
use malachite_nz_test_util::generators::{
    integer_gen, integer_pair_gen, integer_triple_gen, natural_pair_gen,
};
use num::BigInt;
use std::str::FromStr;

#[test]
fn test_mul() {
    let test = |s, t, out| {
        let u = Integer::from_str(s).unwrap();
        let v = Integer::from_str(t).unwrap();

        let mut n = u.clone();
        n *= v.clone();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = u.clone();
        n *= &v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = u.clone() * v.clone();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &u * v.clone();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = u.clone() * &v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &u * &v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = BigInt::from_str(s).unwrap() * BigInt::from_str(t).unwrap();
        assert_eq!(n.to_string(), out);

        let n = rug::Integer::from_str(s).unwrap() * rug::Integer::from_str(t).unwrap();
        assert_eq!(n.to_string(), out);
    };
    test("0", "0", "0");
    test("0", "123", "0");
    test("0", "-123", "0");
    test("123", "0", "0");
    test("-123", "0", "0");
    test("1", "123", "123");
    test("1", "-123", "-123");
    test("-1", "123", "-123");
    test("-1", "-123", "123");
    test("123", "1", "123");
    test("123", "-1", "-123");
    test("-123", "1", "-123");
    test("-123", "-1", "123");
    test("123", "456", "56088");
    test("123", "-456", "-56088");
    test("-123", "456", "-56088");
    test("-123", "-456", "56088");
    test("0", "1000000000000", "0");
    test("0", "-1000000000000", "0");
    test("1000000000000", "0", "0");
    test("-1000000000000", "0", "0");
    test("1", "1000000000000", "1000000000000");
    test("1", "-1000000000000", "-1000000000000");
    test("-1", "1000000000000", "-1000000000000");
    test("-1", "-1000000000000", "1000000000000");
    test("1000000000000", "1", "1000000000000");
    test("1000000000000", "-1", "-1000000000000");
    test("-1000000000000", "1", "-1000000000000");
    test("-1000000000000", "-1", "1000000000000");
    test("1000000000000", "123", "123000000000000");
    test("1000000000000", "-123", "-123000000000000");
    test("-1000000000000", "123", "-123000000000000");
    test("-1000000000000", "-123", "123000000000000");
    test("123", "1000000000000", "123000000000000");
    test("123", "-1000000000000", "-123000000000000");
    test("-123", "1000000000000", "-123000000000000");
    test("-123", "-1000000000000", "123000000000000");
    test("123456789000", "987654321000", "121932631112635269000000");
    test("123456789000", "-987654321000", "-121932631112635269000000");
    test("-123456789000", "987654321000", "-121932631112635269000000");
    test("-123456789000", "-987654321000", "121932631112635269000000");
    test("4294967295", "2", "8589934590");
    test("4294967295", "-2", "-8589934590");
    test("-4294967295", "2", "-8589934590");
    test("-4294967295", "-2", "8589934590");
    test("4294967295", "4294967295", "18446744065119617025");
    test("4294967295", "-4294967295", "-18446744065119617025");
    test("-4294967295", "4294967295", "-18446744065119617025");
    test("-4294967295", "-4294967295", "18446744065119617025");
    test("18446744073709551615", "2", "36893488147419103230");
    test("18446744073709551615", "-2", "-36893488147419103230");
    test("-18446744073709551615", "2", "-36893488147419103230");
    test("-18446744073709551615", "-2", "36893488147419103230");
}

#[test]
fn mul_properties() {
    integer_pair_gen().test_properties(|(x, y)| {
        let product_val_val = x.clone() * y.clone();
        let product_val_ref = x.clone() * &y;
        let product_ref_val = &x * y.clone();
        let product = &x * &y;
        assert!(product_val_val.is_valid());
        assert!(product_val_ref.is_valid());
        assert!(product_ref_val.is_valid());
        assert!(product.is_valid());
        assert_eq!(product_val_val, product);
        assert_eq!(product_val_ref, product);
        assert_eq!(product_ref_val, product);

        let mut mut_x = x.clone();
        mut_x *= y.clone();
        assert!(mut_x.is_valid());
        assert_eq!(mut_x, product);
        let mut mut_x = x.clone();
        mut_x *= &y;
        assert_eq!(mut_x, product);
        assert!(mut_x.is_valid());

        let mut mut_x = integer_to_rug_integer(&x);
        mut_x *= integer_to_rug_integer(&y);
        assert_eq!(rug_integer_to_integer(&mut_x), product);

        assert_eq!(
            bigint_to_integer(&(integer_to_bigint(&x) * integer_to_bigint(&y))),
            product
        );
        assert_eq!(
            rug_integer_to_integer(&(integer_to_rug_integer(&x) * integer_to_rug_integer(&y))),
            product
        );
        assert_eq!(&y * &x, product);
        if x != 0 {
            let (q, r) = (&product).div_mod(&x);
            assert_eq!(q, y);
            assert_eq!(r, 0);
        }
        if y != 0 {
            let (q, r) = (&product).div_mod(&y);
            assert_eq!(q, x);
            assert_eq!(r, 0);
        }

        assert_eq!(-&x * &y, -&product);
        assert_eq!(x * -y, -product);
    });

    integer_gen().test_properties(|ref x| {
        assert_eq!(x * Integer::ZERO, 0);
        assert_eq!(Integer::ZERO * x, 0);
        assert_eq!(x * Integer::ONE, *x);
        assert_eq!(Integer::ONE * x, *x);
        assert_eq!(x * Integer::NEGATIVE_ONE, -x);
        assert_eq!(Integer::NEGATIVE_ONE * x, -x);
        assert_eq!(x * x, x.square());
    });

    integer_triple_gen().test_properties(|(ref x, ref y, ref z)| {
        assert_eq!((x * y) * z, x * (y * z));
        assert_eq!(x * (y + z), x * y + x * z);
        assert_eq!((x + y) * z, x * z + y * z);
    });

    natural_pair_gen().test_properties(|(x, y)| {
        assert_eq!(&x * &y, Integer::from(x) * Integer::from(y));
    });

    signed_pair_gen::<SignedLimb>().test_properties(|(x, y)| {
        assert_eq!(
            Integer::from(SignedDoubleLimb::from(x) * SignedDoubleLimb::from(y)),
            Integer::from(x) * Integer::from(y)
        );
    });
}
