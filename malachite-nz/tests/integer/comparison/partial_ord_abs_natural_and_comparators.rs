use std::cmp::Ordering;
use std::str::FromStr;

use malachite_base::num::comparison::traits::PartialOrdAbs;

use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;

#[test]
fn test_partial_ord_integer_natural() {
    let test = |x, y, cmp, lt: bool, gt: bool, le: bool, ge: bool| {
        assert_eq!(
            Integer::from_str(x)
                .unwrap()
                .partial_cmp_abs(&Natural::from_str(y).unwrap()),
            cmp
        );
        assert_eq!(
            Natural::from_str(y)
                .unwrap()
                .partial_cmp_abs(&Integer::from_str(x).unwrap())
                .map(|o| o.reverse()),
            cmp
        );
        assert_eq!(
            lt,
            Integer::from_str(x)
                .unwrap()
                .lt_abs(&Natural::from_str(y).unwrap())
        );
        assert_eq!(
            gt,
            Integer::from_str(x)
                .unwrap()
                .gt_abs(&Natural::from_str(y).unwrap())
        );
        assert_eq!(
            le,
            Integer::from_str(x)
                .unwrap()
                .le_abs(&Natural::from_str(y).unwrap())
        );
        assert_eq!(
            ge,
            Integer::from_str(x)
                .unwrap()
                .ge_abs(&Natural::from_str(y).unwrap())
        );
        assert_eq!(
            lt,
            Natural::from_str(y)
                .unwrap()
                .gt_abs(&Integer::from_str(x).unwrap())
        );
        assert_eq!(
            gt,
            Natural::from_str(y)
                .unwrap()
                .lt_abs(&Integer::from_str(x).unwrap())
        );
        assert_eq!(
            le,
            Natural::from_str(y)
                .unwrap()
                .ge_abs(&Integer::from_str(x).unwrap())
        );
        assert_eq!(
            ge,
            Natural::from_str(y)
                .unwrap()
                .le_abs(&Integer::from_str(x).unwrap())
        );
    };
    test("0", "0", Some(Ordering::Equal), false, false, true, true);
    test("0", "5", Some(Ordering::Less), true, false, true, false);
    test(
        "123",
        "123",
        Some(Ordering::Equal),
        false,
        false,
        true,
        true,
    );
    test("123", "124", Some(Ordering::Less), true, false, true, false);
    test(
        "123",
        "122",
        Some(Ordering::Greater),
        false,
        true,
        false,
        true,
    );
    test(
        "1000000000000",
        "123",
        Some(Ordering::Greater),
        false,
        true,
        false,
        true,
    );
    test(
        "123",
        "1000000000000",
        Some(Ordering::Less),
        true,
        false,
        true,
        false,
    );
    test(
        "1000000000000",
        "1000000000000",
        Some(Ordering::Equal),
        false,
        false,
        true,
        true,
    );
    test(
        "-1000000000000",
        "1000000000000",
        Some(Ordering::Equal),
        false,
        false,
        true,
        true,
    );
    test(
        "-1000000000000",
        "0",
        Some(Ordering::Greater),
        false,
        true,
        false,
        true,
    );
}
