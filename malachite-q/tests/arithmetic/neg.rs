use malachite_base::num::arithmetic::traits::NegAssign;
use malachite_nz_test_util::generators::integer_gen;
use malachite_q::Rational;
use malachite_q_test_util::common::{
    bigrational_to_rational, rational_to_bigrational, rational_to_rug_rational,
    rug_rational_to_rational,
};
use malachite_q_test_util::generators::rational_gen;
use num::BigRational;
use std::str::FromStr;

#[test]
fn test_neg() {
    let test = |s, out| {
        let x = Rational::from_str(s).unwrap();

        let neg = -x.clone();
        assert!(neg.is_valid());
        assert_eq!(neg.to_string(), out);

        let neg = -&x;
        assert!(neg.is_valid());
        assert_eq!(neg.to_string(), out);

        assert_eq!((-BigRational::from_str(s).unwrap()).to_string(), out);
        assert_eq!((-rug::Rational::from_str(s).unwrap()).to_string(), out);

        let mut x = x;
        x.neg_assign();
        assert!(neg.is_valid());
        assert_eq!(x.to_string(), out);
    };
    test("0", "0");
    test("123", "-123");
    test("-123", "123");
    test("1000000000000", "-1000000000000");
    test("-1000000000000", "1000000000000");
    test("3000000000", "-3000000000");
    test("-3000000000", "3000000000");
}

#[test]
fn abs_properties() {
    rational_gen().test_properties(|x| {
        let neg = -x.clone();
        assert!(neg.is_valid());

        assert_eq!(bigrational_to_rational(&-rational_to_bigrational(&x)), neg);

        assert_eq!(
            rug_rational_to_rational(&-rational_to_rug_rational(&x)),
            neg
        );

        let neg_alt = -&x;
        assert!(neg_alt.is_valid());
        assert_eq!(neg_alt, neg);

        let mut neg_alt = x.clone();
        neg_alt.neg_assign();
        assert!(neg_alt.is_valid());
        assert_eq!(neg_alt, neg);

        assert_eq!(neg == x, x == 0);
        assert_eq!(-&neg, x);
        assert_eq!(x + neg, 0);
    });

    integer_gen().test_properties(|x| {
        assert_eq!(-Rational::from(&x), Rational::from(-x));
    });
}
