use malachite_base::num::arithmetic::traits::DivRound;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{
    ExactFrom, PowerOfTwoDigitIterable, PowerOfTwoDigitIterator, PowerOfTwoDigits,
};
use malachite_base::rounding_modes::RoundingMode;
use malachite_base_test_util::generators::{
    unsigned_pair_gen_var_4, unsigned_pair_gen_var_5, unsigned_triple_gen_var_3,
    unsigned_unsigned_bool_vec_triple_gen_var_1,
};
use std::panic::catch_unwind;

#[test]
pub fn test_power_of_two_digits() {
    assert_eq!(
        PowerOfTwoDigits::<u64>::to_power_of_two_digits_asc(&107u32, 2),
        &[3, 2, 2, 1]
    );
    let mut digits = PowerOfTwoDigitIterable::<u8>::power_of_two_digits(107u32, 2);
    assert_eq!(digits.next(), Some(3));
    assert_eq!(digits.next_back(), Some(1));
    assert_eq!(digits.next_back(), Some(2));
    assert_eq!(digits.next(), Some(2));
    assert_eq!(digits.next(), None);
    assert_eq!(digits.next_back(), None);

    assert_eq!(digits.get(0), 3);
    assert_eq!(digits.get(1), 2);
    assert_eq!(digits.get(2), 2);
    assert_eq!(digits.get(3), 1);
    assert_eq!(digits.get(4), 0);
    assert_eq!(digits.get(5), 0);

    let mut digits = PowerOfTwoDigitIterable::<u8>::power_of_two_digits(107u32, 2);
    assert_eq!(digits.next_back(), Some(1));
    assert_eq!(digits.next(), Some(3));
    assert_eq!(digits.next(), Some(2));
    assert_eq!(digits.next(), Some(2));
    assert_eq!(digits.next(), None);
    assert_eq!(digits.next_back(), None);

    let mut digits = PowerOfTwoDigitIterable::<u32>::power_of_two_digits(0u8, 5);
    assert_eq!(digits.next(), None);
    assert_eq!(digits.next_back(), None);

    assert_eq!(
        PowerOfTwoDigits::<u64>::to_power_of_two_digits_asc(&105u32, 1),
        &[1, 0, 0, 1, 0, 1, 1]
    );
    let mut digits = PowerOfTwoDigitIterable::<u8>::power_of_two_digits(105u32, 1);
    assert_eq!(digits.next(), Some(1));
    assert_eq!(digits.next_back(), Some(1));
    assert_eq!(digits.next_back(), Some(1));
    assert_eq!(digits.next_back(), Some(0));
    assert_eq!(digits.next(), Some(0));
    assert_eq!(digits.next(), Some(0));
    assert_eq!(digits.next(), Some(1));
    assert_eq!(digits.next(), None);
    assert_eq!(digits.next_back(), None);

    assert_eq!(digits.get(0), 1);
    assert_eq!(digits.get(1), 0);
    assert_eq!(digits.get(2), 0);
    assert_eq!(digits.get(3), 1);
    assert_eq!(digits.get(4), 0);
    assert_eq!(digits.get(5), 1);
    assert_eq!(digits.get(6), 1);
    assert_eq!(digits.get(7), 0);
    assert_eq!(digits.get(8), 0);

    let mut digits = PowerOfTwoDigitIterable::<u8>::power_of_two_digits(105u32, 1);
    assert_eq!(digits.next_back(), Some(1));
    assert_eq!(digits.next(), Some(1));
    assert_eq!(digits.next(), Some(0));
    assert_eq!(digits.next(), Some(0));
    assert_eq!(digits.next_back(), Some(1));
    assert_eq!(digits.next_back(), Some(0));
    assert_eq!(digits.next_back(), Some(1));
    assert_eq!(digits.next(), None);
    assert_eq!(digits.next_back(), None);
}

fn power_of_two_digits_fail_helper<
    T: PowerOfTwoDigitIterable<U> + PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>() {
    assert_panic!(PowerOfTwoDigitIterable::<U>::power_of_two_digits(
        T::exact_from(107),
        0
    ));
    assert_panic!(PowerOfTwoDigitIterable::<U>::power_of_two_digits(
        T::exact_from(107),
        200
    ));
}

#[test]
fn power_of_two_digits_fail() {
    apply_fn_to_unsigneds_and_unsigneds!(power_of_two_digits_fail_helper);
}

fn power_of_two_digit_iterable_helper<
    T: PowerOfTwoDigitIterable<U> + PowerOfTwoDigits<U> + PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>() {
    unsigned_pair_gen_var_4::<T, U>().test_properties(|(u, log_base)| {
        let significant_digits = usize::exact_from(
            u.significant_bits()
                .div_round(log_base, RoundingMode::Ceiling),
        );
        assert_eq!(
            PowerOfTwoDigitIterable::<U>::power_of_two_digits(u, log_base).size_hint(),
            (significant_digits, Some(significant_digits))
        );
    });

    unsigned_unsigned_bool_vec_triple_gen_var_1::<T, U>().test_properties(
        |(u, log_base, ref bs)| {
            let mut digits = PowerOfTwoDigitIterable::<U>::power_of_two_digits(u, log_base);
            let mut digit_vec = Vec::new();
            let mut i = 0;
            for &b in bs {
                if b {
                    digit_vec.insert(i, digits.next().unwrap());
                    i += 1;
                } else {
                    digit_vec.insert(i, digits.next_back().unwrap())
                }
            }
            assert!(digits.next().is_none());
            assert!(digits.next_back().is_none());
            assert_eq!(
                PowerOfTwoDigits::<U>::to_power_of_two_digits_asc(&u, log_base),
                digit_vec
            );
        },
    );

    unsigned_triple_gen_var_3::<T, U, u64>().test_properties(|(u, log_base, i)| {
        let digits = PowerOfTwoDigitIterable::<U>::power_of_two_digits(u, log_base);
        if i < u
            .significant_bits()
            .div_round(log_base, RoundingMode::Ceiling)
        {
            assert_eq!(
                digits.get(i),
                PowerOfTwoDigits::<U>::to_power_of_two_digits_asc(&u, log_base)
                    [usize::exact_from(i)]
            );
        } else {
            assert_eq!(digits.get(i), U::ZERO);
        }
    });

    unsigned_pair_gen_var_5::<u64, U>().test_properties(|(i, log_base)| {
        let digits = PowerOfTwoDigitIterable::<U>::power_of_two_digits(T::ZERO, log_base);
        assert_eq!(digits.get(i), U::ZERO);
    });
}

#[test]
fn power_of_two_digit_iterable_properties() {
    apply_fn_to_unsigneds_and_unsigneds!(power_of_two_digit_iterable_helper);
}