use itertools::Itertools;
use malachite_base::strings::ExtraToString;
use malachite_q::exhaustive::exhaustive_non_negative_rationals;

#[test]
fn test_exhaustive_non_negative_rationals() {
    assert_eq!(
        exhaustive_non_negative_rationals()
            .take(20)
            .collect_vec()
            .to_string(),
        "[0, 1, 1/2, 2, 1/3, 3/2, 2/3, 3, 1/4, 4/3, 3/5, 5/2, 2/5, 5/3, 3/4, 4, 1/5, 5/4, 4/7, \
        7/3]"
    )
}
