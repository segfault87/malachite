use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::generators::{signed_gen, signed_gen_var_2, unsigned_gen};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::common::{bigint_to_integer, rug_integer_to_integer};
use num::BigInt;
use rug;

#[test]
fn test_from_u32() {
    let test = |u: u32, out| {
        let x = Integer::from(u);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());
        assert_eq!(BigInt::from(u).to_string(), out);
        assert_eq!(rug::Integer::from(u).to_string(), out);
    };
    test(0, "0");
    test(123, "123");
    test(u32::MAX, "4294967295");
}

#[test]
fn test_from_u64() {
    let test = |u: u64, out| {
        let x = Integer::from(u);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());
        assert_eq!(BigInt::from(u).to_string(), out);
        assert_eq!(rug::Integer::from(u).to_string(), out);
    };
    test(0, "0");
    test(123, "123");
    test(u64::MAX, "18446744073709551615");
}

#[test]
fn test_from_i32() {
    let test = |i: i32, out| {
        let x = Integer::from(i);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());
        assert_eq!(BigInt::from(i).to_string(), out);
        assert_eq!(rug::Integer::from(i).to_string(), out);
    };
    test(0, "0");
    test(123, "123");
    test(-123, "-123");
    test(i32::MIN, "-2147483648");
    test(i32::MAX, "2147483647");
}

#[test]
fn test_from_i64() {
    let test = |i: i64, out| {
        let x = Integer::from(i);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());
        assert_eq!(BigInt::from(i).to_string(), out);
        assert_eq!(rug::Integer::from(i).to_string(), out);
    };
    test(0, "0");
    test(123, "123");
    test(-123, "-123");
    test(i64::MIN, "-9223372036854775808");
    test(i64::MAX, "9223372036854775807");
}

fn from_unsigned_properties_helper<T: for<'a> TryFrom<&'a Integer> + PrimitiveUnsigned>()
where
    Integer: From<T>,
    Natural: From<T>,
    u128: TryFrom<T>,
{
    unsigned_gen::<T>().test_properties(|u| {
        let n = Integer::from(u);
        assert!(n.is_valid());
        assert_eq!(T::exact_from(&n), u);
        let alt_n: Integer = From::from(Natural::from(u));
        assert_eq!(alt_n, n);
        let alt_n: Integer = From::from(u128::exact_from(u));
        assert_eq!(alt_n, n);
    });
}

fn from_signed_properties_helper<T: for<'a> TryFrom<&'a Integer> + PrimitiveSigned>()
where
    Integer: From<T>,
    Natural: TryFrom<T>,
    i128: TryFrom<T>,
{
    signed_gen::<T>().test_properties(|i| {
        let n = Integer::from(i);
        assert!(n.is_valid());
        assert_eq!(T::exact_from(&n), i);
        let alt_n: Integer = From::from(i128::exact_from(i));
        assert_eq!(alt_n, n);
    });

    signed_gen_var_2::<T>().test_properties(|i| {
        let n: Integer = From::from(Natural::exact_from(i));
        assert_eq!(n, Integer::from(i));
    });
}

#[test]
fn from_primitive_int_properties() {
    apply_fn_to_unsigneds!(from_unsigned_properties_helper);
    apply_fn_to_signeds!(from_signed_properties_helper);

    unsigned_gen::<u32>().test_properties(|u| {
        let n = Integer::from(u);
        assert_eq!(bigint_to_integer(&BigInt::from(u)), n);
        assert_eq!(rug_integer_to_integer(&rug::Integer::from(u)), n);
    });

    unsigned_gen::<u64>().test_properties(|u| {
        let n = Integer::from(u);
        assert_eq!(bigint_to_integer(&BigInt::from(u)), n);
        assert_eq!(rug_integer_to_integer(&rug::Integer::from(u)), n);
    });

    signed_gen::<i32>().test_properties(|i| {
        let n = Integer::from(i);
        assert_eq!(bigint_to_integer(&BigInt::from(i)), n);
        assert_eq!(rug_integer_to_integer(&rug::Integer::from(i)), n);
    });

    signed_gen::<i64>().test_properties(|i| {
        let n = Integer::from(i);
        assert_eq!(bigint_to_integer(&BigInt::from(i)), n);
        assert_eq!(rug_integer_to_integer(&rug::Integer::from(i)), n);
    });
}
