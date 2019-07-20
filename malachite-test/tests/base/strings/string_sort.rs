use malachite_base::strings::{string_is_subset, string_nub, string_sort};

use common::test_properties_no_special;
use malachite_test::inputs::base::strings;

#[test]
fn string_sort_properties() {
    test_properties_no_special(strings, |s| {
        //TODO is ascending
        let t = string_sort(s);
        assert_eq!(s.len(), t.len());
        assert_eq!(string_sort(&t), t);
        assert_eq!(string_nub(&t), string_sort(&string_nub(s)));
        assert!(string_is_subset(s, &t));
        assert!(string_is_subset(&t, s));
    });
}