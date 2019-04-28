use std::char;

use malachite_base::chars::{
    char_to_contiguous_range, char_to_digit, contiguous_range_to_char, digit_to_char,
    CHAR_JUST_ABOVE_SURROGATES, CHAR_JUST_BELOW_SURROGATES, NUMBER_OF_CHARS,
};

use common::{test_properties_no_limit_exhaustive_no_special, test_properties_no_special};
use malachite_test::inputs::base::{chars, pairs_of_chars};

//TODO move to proper location

#[test]
fn test_digit_to_char() {
    let test = |i, out| assert_eq!(digit_to_char(i), out);
    test(0, Some('0'));
    test(6, Some('6'));
    test(10, Some('a'));
    test(20, Some('k'));
    test(35, Some('z'));
    test(36, None);
}

#[test]
fn test_char_to_digit() {
    let test = |c, out| assert_eq!(char_to_digit(c), out);
    test('0', Some(0));
    test('6', Some(6));
    test('a', Some(10));
    test('k', Some(20));
    test('z', Some(35));
    test(' ', None);
    test('A', None);
}

#[test]
fn test_char_to_contiguous_range() {
    let test = |c, out| {
        assert_eq!(char_to_contiguous_range(c), out);
    };
    test('\u{0}', 0);
    test('a', 97);
    test('A', 65);
    test(CHAR_JUST_BELOW_SURROGATES, 55_295);
    test(CHAR_JUST_ABOVE_SURROGATES, 55_296);
    test(char::MAX, 1_112_063);
}

#[test]
fn char_to_contiguous_range_properties() {
    test_properties_no_limit_exhaustive_no_special(chars, |&c| {
        let u = char_to_contiguous_range(c);
        assert_eq!(contiguous_range_to_char(u), Some(c));
        assert!(u < NUMBER_OF_CHARS);
    });

    test_properties_no_special(pairs_of_chars, |&(c, d)| {
        assert_eq!(
            c.cmp(&d),
            char_to_contiguous_range(c).cmp(&char_to_contiguous_range(d))
        );
    });
}
