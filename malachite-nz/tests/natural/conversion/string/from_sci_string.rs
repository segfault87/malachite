use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::string::options::FromSciStringOptions;
use malachite_base::num::conversion::traits::{ExactFrom, FromSciString, ToStringBase};
use malachite_base::rounding_modes::RoundingMode;
use malachite_base::test_util::generators::{
    string_from_sci_string_options_pair_gen_var_2, string_from_sci_string_options_pair_gen_var_3,
    string_gen_var_14, string_gen_var_15,
};
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::natural_gen;
use std::str::FromStr;

#[test]
pub fn test_from_sci_string() {
    fn test(s: &str, out: Option<&'static str>) {
        let out = out.map(|s| Natural::from_str(s).unwrap());
        assert_eq!(Natural::from_sci_string(s), out);
        assert_eq!(
            Natural::from_sci_string_with_options(s, FromSciStringOptions::default()),
            out
        );
    }
    test("0", Some("0"));
    test("00", Some("0"));
    test("+0", Some("0"));
    test("-0", Some("0"));
    test("0.00", Some("0"));
    test("0e1", Some("0"));
    test("0e+1", Some("0"));
    test("0e-1", Some("0"));
    test("+0e+1", Some("0"));
    test("-0e+1", Some("0"));
    test("+0.0e+1", Some("0"));
    test("-0.0e+1", Some("0"));
    test(".0", Some("0"));
    test(".00", Some("0"));
    test(".00e0", Some("0"));
    test(".00e1", Some("0"));
    test(".00e-1", Some("0"));
    test("-.0", Some("0"));
    test("-.00", Some("0"));
    test("-.00e0", Some("0"));
    test("-.00e1", Some("0"));
    test("-.00e-1", Some("0"));
    test("+.0", Some("0"));
    test("+.00", Some("0"));
    test("+.00e0", Some("0"));
    test("+.00e1", Some("0"));
    test("+.00e-1", Some("0"));

    test("123", Some("123"));
    test("00123", Some("123"));
    test("+123", Some("123"));
    test("123.00", Some("123"));
    test("123e0", Some("123"));
    test("12.3e1", Some("123"));
    test("1.23e2", Some("123"));
    test("1.23E2", Some("123"));
    test("1.23e+2", Some("123"));
    test("1.23E+2", Some("123"));
    test(".123e3", Some("123"));
    test("0.123e3", Some("123"));
    test("+0.123e3", Some("123"));
    test("0.0123e4", Some("123"));
    test("1230e-1", Some("123"));
    test("12300e-2", Some("123"));
    test("12300E-2", Some("123"));

    test("123.4", Some("123"));
    test("123.8", Some("124"));
    test("123.5", Some("124"));
    test("124.5", Some("124"));
    test("127.49", Some("127"));

    test("", None);
    test("+", None);
    test("-", None);
    test("10e", None);
    test("++1", None);
    test("1.0.0", None);
    test("1e++1", None);
    test("1e0.1", None);
    test("--.0", None);
    test("++.0", None);
    test(".+2", None);
    test(".-2", None);
    test("0.000a", None);
    test("0.00ae-10", None);
    test("0e10000000000000000000000000000", None);
    test("0e-10000000000000000000000000000", None);
}

#[test]
pub fn test_from_sci_string_with_options() {
    fn test(s: &str, options: FromSciStringOptions, out: Option<&str>) {
        let out = out.map(|s| Natural::from_str(s).unwrap());
        assert_eq!(Natural::from_sci_string_with_options(s, options), out);
    }
    fn test_u<T: PrimitiveUnsigned>(s: &str, options: FromSciStringOptions, out: Option<T>)
    where
        Natural: From<T>,
    {
        let out = out.map(Natural::from);
        assert_eq!(Natural::from_sci_string_with_options(s, options), out);
    }
    fn test_i<T: PrimitiveSigned>(s: &str, options: FromSciStringOptions, out: Option<T>)
    where
        Natural: TryFrom<T>,
    {
        let out = out.map(Natural::exact_from);
        assert_eq!(Natural::from_sci_string_with_options(s, options), out);
    }
    // For tests with the default options, see `test_from_sci_string`

    let mut options = FromSciStringOptions::default();
    options.set_base(2);
    test_u(
        "11111111111111111111111111111111111111111111111111111111111111111111111111111111111111111\
        111111111111111111111111111111111111111",
        options,
        Some(u128::MAX),
    );
    options.set_base(3);
    test_u(
        "202201102121002021012000211012011021221022212021111001022110211020010021100121010",
        options,
        Some(u128::MAX),
    );
    options.set_base(4);
    test_u(
        "3333333333333333333333333333333333333333333333333333333333333333",
        options,
        Some(u128::MAX),
    );
    options.set_base(5);
    test_u(
        "11031110441201303134210404233413032443021130230130231310",
        options,
        Some(u128::MAX),
    );
    options.set_base(8);
    test_u(
        "3777777777777777777777777777777777777777777",
        options,
        Some(u128::MAX),
    );
    options.set_base(16);
    test_u("ffffffffffffffffffffffffffffffff", options, Some(u128::MAX));
    options.set_base(32);
    test_u("7vvvvvvvvvvvvvvvvvvvvvvvvv", options, Some(u128::MAX));
    options.set_base(36);
    test_u("f5lxx1zz5pnorynqglhzmsp33", options, Some(u128::MAX));

    options.set_base(2);
    test_i(
        "11111111111111111111111111111111111111111111111111111111111111111111111111111111111111111\
        11111111111111111111111111111111111111",
        options,
        Some(i128::MAX),
    );
    options.set_base(3);
    test_i(
        "101100201022001010121000102002120122110122221010202000122201220121120010200022001",
        options,
        Some(i128::MAX),
    );
    options.set_base(4);
    test_i(
        "1333333333333333333333333333333333333333333333333333333333333333",
        options,
        Some(i128::MAX),
    );
    options.set_base(5);
    test_i(
        "3013030220323124042102424341431241221233040112312340402",
        options,
        Some(i128::MAX),
    );
    options.set_base(8);
    test_i(
        "1777777777777777777777777777777777777777777",
        options,
        Some(i128::MAX),
    );
    options.set_base(16);
    test_i("7fffffffffffffffffffffffffffffff", options, Some(i128::MAX));
    options.set_base(32);
    test_i("3vvvvvvvvvvvvvvvvvvvvvvvvv", options, Some(i128::MAX));
    options.set_base(36);
    test_i("7ksyyizzkutudzbv8aqztecjj", options, Some(i128::MAX));

    options.set_base(2);
    test("1e+5", options, Some("32"));
    test("1e5", options, Some("32"));
    options.set_base(3);
    test("1e+5", options, Some("243"));
    test("1e5", options, Some("243"));
    options.set_base(4);
    test("1e+5", options, Some("1024"));
    test("1e5", options, Some("1024"));
    options.set_base(5);
    test("1e+5", options, Some("3125"));
    test("1e5", options, Some("3125"));
    options.set_base(8);
    test("1e+5", options, Some("32768"));
    test("1e5", options, Some("32768"));
    options.set_base(16);
    test("1e+5", options, Some("1048576"));
    test("1e5", options, Some("485"));
    options.set_base(32);
    test("1e+5", options, Some("33554432"));
    test("1e5", options, Some("1477"));
    options.set_base(36);
    test("1e+5", options, Some("60466176"));
    test("1E+5", options, Some("60466176"));
    test("1e5", options, Some("1805"));

    options.set_base(16);
    test("ff", options, Some("255"));
    test("fF", options, Some("255"));
    test("Ff", options, Some("255"));
    test("FF", options, Some("255"));

    options = FromSciStringOptions::default();
    options.set_rounding_mode(RoundingMode::Down);
    test("123.4", options, Some("123"));
    options.set_rounding_mode(RoundingMode::Floor);
    test("123.4", options, Some("123"));
    options.set_rounding_mode(RoundingMode::Up);
    test("123.4", options, Some("124"));
    options.set_rounding_mode(RoundingMode::Ceiling);
    test("123.4", options, Some("124"));
    options.set_rounding_mode(RoundingMode::Nearest);
    test("123.4", options, Some("123"));
    options.set_rounding_mode(RoundingMode::Exact);
    test("123.4", options, None);

    options.set_rounding_mode(RoundingMode::Down);
    test("123.5", options, Some("123"));
    options.set_rounding_mode(RoundingMode::Floor);
    test("123.5", options, Some("123"));
    options.set_rounding_mode(RoundingMode::Up);
    test("123.5", options, Some("124"));
    options.set_rounding_mode(RoundingMode::Ceiling);
    test("123.5", options, Some("124"));
    options.set_rounding_mode(RoundingMode::Nearest);
    test("123.5", options, Some("124"));
    options.set_rounding_mode(RoundingMode::Exact);
    test("123.5", options, None);

    options.set_rounding_mode(RoundingMode::Down);
    test("0.4", options, Some("0"));
    options.set_rounding_mode(RoundingMode::Floor);
    test("0.4", options, Some("0"));
    options.set_rounding_mode(RoundingMode::Up);
    test("0.4", options, Some("1"));
    options.set_rounding_mode(RoundingMode::Ceiling);
    test("0.4", options, Some("1"));
    options.set_rounding_mode(RoundingMode::Nearest);
    test("0.4", options, Some("0"));
    options.set_rounding_mode(RoundingMode::Exact);
    test("0.4", options, None);

    options.set_rounding_mode(RoundingMode::Down);
    test("0.04", options, Some("0"));
    options.set_rounding_mode(RoundingMode::Floor);
    test("0.04", options, Some("0"));
    options.set_rounding_mode(RoundingMode::Up);
    test("0.04", options, Some("1"));
    options.set_rounding_mode(RoundingMode::Ceiling);
    test("0.04", options, Some("1"));
    options.set_rounding_mode(RoundingMode::Nearest);
    test("0.04", options, Some("0"));
    options.set_rounding_mode(RoundingMode::Exact);
    test("0.04", options, None);

    options = FromSciStringOptions::default();
    options.set_base(2);
    // 1/2 is 0.1
    test("1.01", options, Some("1"));
    test("1.1", options, Some("2"));
    test("1.11", options, Some("2"));
    test("0.01", options, Some("0"));
    test("0.1", options, Some("0"));
    test("0.11", options, Some("1"));
    options.set_base(3);
    // 1/2 is 0.111...
    test("1.1", options, Some("1"));
    test("1.11", options, Some("1"));
    test("1.111", options, Some("1"));
    test("1.112", options, Some("2"));
    test("0.1", options, Some("0"));
    test("0.11", options, Some("0"));
    test("0.111", options, Some("0"));
    test("0.112", options, Some("1"));

    options = FromSciStringOptions::default();
    options.set_base(2);
    test("2", options, None);
    test("102", options, None);
    test("12e4", options, None);
    test("12e-4", options, None);
    test("1.2", options, None);
    test("0.2", options, None);
    test("0.002", options, None);

    options = FromSciStringOptions::default();
    options.set_rounding_mode(RoundingMode::Exact);
    test("1.5", options, None);
    test("1.9999999999999999999999999999", options, None);

    options.set_base(2);
    test_i(
        "11111111111111111111111111111111111111111111111111111111111111111111111111111111111111111\
        11111111111111111111111111111111111111",
        options,
        Some(i128::MAX),
    );
    options.set_base(3);
    test_i(
        "101100201022001010121000102002120122110122221010202000122201220121120010200022001",
        options,
        Some(i128::MAX),
    );
    options.set_base(4);
    test_i(
        "1333333333333333333333333333333333333333333333333333333333333333",
        options,
        Some(i128::MAX),
    );
    options.set_base(5);
    test_i(
        "3013030220323124042102424341431241221233040112312340402",
        options,
        Some(i128::MAX),
    );
    options.set_base(8);
    test_i(
        "1777777777777777777777777777777777777777777",
        options,
        Some(i128::MAX),
    );
    options.set_base(16);
    test_i("7fffffffffffffffffffffffffffffff", options, Some(i128::MAX));
    options.set_base(32);
    test_i("3vvvvvvvvvvvvvvvvvvvvvvvvv", options, Some(i128::MAX));
    options.set_base(36);
    test_i("7ksyyizzkutudzbv8aqztecjj", options, Some(i128::MAX));

    options.set_rounding_mode(RoundingMode::Down);
    test("-0.4", options, Some("0"));
    options.set_rounding_mode(RoundingMode::Floor);
    test("-0.4", options, None);
    options.set_rounding_mode(RoundingMode::Up);
    test("-0.4", options, None);
    options.set_rounding_mode(RoundingMode::Ceiling);
    test("-0.4", options, Some("0"));
    options.set_rounding_mode(RoundingMode::Nearest);
    test("-0.4", options, Some("0"));
    options.set_rounding_mode(RoundingMode::Exact);
    test("-0.4", options, None);

    options.set_rounding_mode(RoundingMode::Down);
    test("-0.04", options, Some("0"));
    options.set_rounding_mode(RoundingMode::Floor);
    test("-0.04", options, None);
    options.set_rounding_mode(RoundingMode::Up);
    test("-0.04", options, None);
    options.set_rounding_mode(RoundingMode::Ceiling);
    test("-0.04", options, Some("0"));
    options.set_rounding_mode(RoundingMode::Nearest);
    test("-0.04", options, Some("0"));
    options.set_rounding_mode(RoundingMode::Exact);
    test("-0.04", options, None);
}

fn from_sci_string_helper(s: &str) {
    if let Some(x) = Natural::from_sci_string(s) {
        assert!(x.is_valid());
        for c in ['.', 'e', 'E', '+'] {
            if s.contains(c) {
                return;
            }
        }
        if s.starts_with('0') || s.starts_with("-0") {
            return;
        }
        assert_eq!(x.to_string(), s);
    }
}

#[test]
fn from_sci_string_properties() {
    string_gen_var_14().test_properties(|s| {
        from_sci_string_helper(&s);
    });

    string_gen_var_15().test_properties(|s| {
        from_sci_string_helper(&s);
    });

    natural_gen().test_properties(|x| {
        assert_eq!(Natural::from_sci_string(&x.to_string()).unwrap(), x);
    });
}

fn from_sci_string_with_options_helper(s: &str, options: FromSciStringOptions) {
    if let Some(x) = Natural::from_sci_string_with_options(s, options) {
        assert!(x.is_valid());
        assert!(!s.ends_with('+'));
        assert!(!s.ends_with('-'));
        assert!(!s.contains("++"));
        assert!(!s.contains("+-"));
        assert!(!s.contains("-+"));
        assert!(!s.contains("--"));
        assert!(!s.contains("-+"));
        assert!(s.chars().filter(|&c| c == '.').count() <= 1);
        assert!(s.chars().filter(|&c| c == '-').count() <= 2);
        assert!(s.chars().filter(|&c| c == '+').count() <= 2);
        for c in ['.', 'e', 'E', '+'] {
            if s.contains(c) {
                return;
            }
        }
        if s.starts_with('0') || s.starts_with("-0") {
            return;
        }
        assert_eq!(x.to_string_base(options.get_base()), s.to_lowercase());
    }
}

#[test]
fn from_sci_string_with_options_properties() {
    string_from_sci_string_options_pair_gen_var_2().test_properties(|(s, options)| {
        from_sci_string_with_options_helper(&s, options);
    });

    string_from_sci_string_options_pair_gen_var_3().test_properties(|(s, options)| {
        from_sci_string_with_options_helper(&s, options);
    });
}
