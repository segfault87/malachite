use std::cmp::{min, Ordering};

use malachite_base::comparison::Max;
use malachite_base::named::Named;
use malachite_base::num::arithmetic::traits::{
    CheckedLogTwo, DivRound, ModPowerOfTwo, Parity, PowerOfTwo,
};
use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{ExactFrom, FromOtherTypeSlice, WrappingFrom};
use malachite_base::num::logic::traits::{
    BitAccess, BitBlockAccess, LowMask, PowerOfTwoDigits, SignificantBits,
};
use malachite_base::round::RoundingMode;
use malachite_base::slices::slice_trailing_zeros;

use natural::InnerNatural::{Large, Small};
use natural::Natural;
use platform::Limb;

fn reformat_slice<'a, T: PrimitiveUnsigned, U: PrimitiveUnsigned, I>(
    ys: &mut Vec<U>,
    xs: I,
    y_width: u64,
    x_width: u64,
) where
    U: WrappingFrom<T>,
    I: Iterator<Item = &'a T>,
{
    let mut y = U::ZERO;
    let mut remaining_y_bits = y_width;
    for &x in xs {
        let mut x = x;
        let mut remaining_x_bits = x_width;
        while remaining_x_bits != 0 {
            let y_index = y_width - remaining_y_bits;
            if remaining_x_bits <= remaining_y_bits {
                y |= U::wrapping_from(x) << y_index;
                remaining_y_bits -= remaining_x_bits;
                remaining_x_bits = 0;
            } else {
                y |= U::wrapping_from(x).mod_power_of_two(remaining_y_bits) << y_index;
                x >>= remaining_y_bits;
                remaining_x_bits -= remaining_y_bits;
                remaining_y_bits = 0;
            }
            if remaining_y_bits == 0 {
                ys.push(y);
                y = U::ZERO;
                remaining_y_bits = y_width;
            }
        }
    }
    if y != U::ZERO {
        ys.push(y);
    }
}

macro_rules! power_of_two_digits_primitive {
    (
        $t: ident,
        $to_power_of_two_digits_asc_naive: ident,
        $from_power_of_two_digits_asc_naive: ident
    ) => {
        impl PowerOfTwoDigits<$t> for Natural {
            /// Returns a `Vec` containing the digits of `self` in ascending order: least- to most-
            /// significant, where the base is a power of two. The base-2 logarithm of the base is
            /// specified. The type of each digit is `$u`, and `log_base` must be no larger than the
            /// width of `$u`. If `self` is 0, the `Vec` is empty; otherwise, it ends with a nonzero
            /// digit.
            ///
            /// Time: worst case O(n)
            ///
            /// Additional memory: worst case O(n)
            ///
            /// where n = `self.significant_bits()`
            ///
            /// # Panics
            /// Panics if `log_base` is greater than the width of `$u`, or if `log_base` is zero.
            ///
            /// # Examples
            /// ```
            /// extern crate malachite_base;
            /// extern crate malachite_nz;
            ///
            /// use malachite_base::num::basic::traits::{Two, Zero};
            /// use malachite_base::num::logic::traits::PowerOfTwoDigits;
            /// use malachite_nz::natural::Natural;
            ///
            /// assert_eq!(
            ///     PowerOfTwoDigits::<u64>::to_power_of_two_digits_asc(&Natural::ZERO, 6),
            ///     Vec::<u64>::new()
            /// );
            /// assert_eq!(
            ///     PowerOfTwoDigits::<u64>::to_power_of_two_digits_asc(&Natural::TWO, 6),
            ///     vec![2]
            /// );
            /// // 123_10 = 173_8
            /// assert_eq!(
            ///     PowerOfTwoDigits::<u16>::to_power_of_two_digits_asc(&Natural::from(123u32), 3),
            ///     vec![3, 7, 1]
            /// );
            /// ```
            fn to_power_of_two_digits_asc(&self, log_base: u64) -> Vec<$t> {
                assert_ne!(log_base, 0);
                if log_base > $t::WIDTH {
                    panic!(
                        "type {:?} is too small for a digit of width {}",
                        $t::NAME,
                        log_base
                    );
                }
                let limbs = match *self {
                    Natural(Small(ref small)) => {
                        return PowerOfTwoDigits::<$t>::to_power_of_two_digits_asc(
                            small,
                            min(log_base, Limb::WIDTH),
                        )
                    }
                    Natural(Large(ref limbs)) => limbs,
                };
                let mut digits = Vec::new();
                if log_base == 1 {
                    let (last, init) = limbs.split_last().unwrap();
                    for limb in init {
                        for i in 0..Limb::WIDTH {
                            digits.push(if limb.get_bit(i) { 1 } else { 0 });
                        }
                    }
                    let mut last = *last;
                    while last != 0 {
                        digits.push(if last.odd() { 1 } else { 0 });
                        last >>= 1;
                    }
                } else if let Some(log_log_base) = log_base.checked_log_two() {
                    match log_log_base.cmp(&Limb::LOG_WIDTH) {
                        Ordering::Equal => {
                            digits.extend(limbs.iter().cloned().map($t::wrapping_from))
                        }
                        Ordering::Less => {
                            for mut limb in limbs.iter().cloned() {
                                let mask = Limb::low_mask(log_base);
                                for _ in 0..u64::power_of_two(Limb::LOG_WIDTH - log_log_base) {
                                    digits.push($t::wrapping_from(limb & mask));
                                    limb >>= log_base;
                                }
                            }
                        }
                        Ordering::Greater => digits.extend(
                            limbs
                                .chunks(usize::power_of_two(log_log_base - Limb::LOG_WIDTH))
                                .map($t::from_other_type_slice),
                        ),
                    }
                } else {
                    reformat_slice(&mut digits, limbs.iter(), log_base, Limb::WIDTH);
                }
                digits.truncate(digits.len() - slice_trailing_zeros(&digits));
                digits
            }

            /// Returns a `Vec` containing the digits of `self` in descending order: most- to least-
            /// significant, where the base is a power of two. The base-2 logarithm of the base is
            /// specified. The type of each digit is `$u`, and `log_base` must be no larger than the
            /// width of `$u`. If `self` is 0, the `Vec` is empty; otherwise, it begins with a
            /// nonzero digit.
            ///
            /// Time: worst case O(n)
            ///
            /// Additional memory: worst case O(n)
            ///
            /// where n = `self.significant_bits()`
            ///
            /// # Panics
            /// Panics if `log_base` is greater than the width of `$u`, or if `log_base` is zero.
            ///
            /// # Examples
            /// ```
            /// extern crate malachite_base;
            /// extern crate malachite_nz;
            ///
            /// use malachite_base::num::basic::traits::{Two, Zero};
            /// use malachite_base::num::logic::traits::PowerOfTwoDigits;
            /// use malachite_nz::natural::Natural;
            ///
            /// assert_eq!(
            ///     PowerOfTwoDigits::<u64>::to_power_of_two_digits_desc(&Natural::ZERO, 6),
            ///     Vec::<u64>::new()
            /// );
            /// assert_eq!(
            ///     PowerOfTwoDigits::<u64>::to_power_of_two_digits_desc(&Natural::TWO, 6),
            ///     vec![2]
            /// );
            /// // 123_10 = 173_8
            /// assert_eq!(
            ///     PowerOfTwoDigits::<u16>::to_power_of_two_digits_desc(&Natural::from(123u32), 3),
            ///     vec![1, 7, 3]
            /// );
            /// ```
            fn to_power_of_two_digits_desc(&self, log_base: u64) -> Vec<$t> {
                let mut digits = self.to_power_of_two_digits_asc(log_base);
                digits.reverse();
                digits
            }

            /// Converts a slice of digits into a `Natural`, where the base is a power of two. The
            /// base-2 logarithm of the base is specified. The input digits are in ascending order:
            /// least- to most-significant. The type of each digit is `$t`, and `log_base` must be
            /// no larger than the width of `$t`.
            ///
            /// Time: worst case O(n)
            ///
            /// Additional memory: worst case O(n)
            ///
            /// where n = `digits.len()`
            ///
            /// # Panics
            /// Panics if `log_base` is greater than the width of `$t`, if `log_base` is zero, or if
            /// some digit is greater than 2<sup>`log_base`.</sup>
            ///
            /// # Examples
            /// ```
            /// extern crate malachite_base;
            /// extern crate malachite_nz;
            ///
            /// use malachite_base::num::logic::traits::PowerOfTwoDigits;
            /// use malachite_nz::natural::Natural;
            ///
            /// let digits: &[u64] = &[0, 0, 0];
            /// assert_eq!(Natural::from_power_of_two_digits_asc(6, digits), 0);
            ///
            /// let digits: &[u64] = &[2, 0];
            /// assert_eq!(Natural::from_power_of_two_digits_asc(6, digits), 2);
            ///
            /// let digits: &[u16] = &[3, 7, 1];
            /// assert_eq!(Natural::from_power_of_two_digits_asc(3, digits), 123);
            /// ```
            #[allow(exceeding_bitshifts)]
            fn from_power_of_two_digits_asc(log_base: u64, digits: &[$t]) -> Natural {
                assert_ne!(log_base, 0);
                if log_base > $t::WIDTH {
                    panic!(
                        "type {:?} is too small for a digit of width {}",
                        $t::NAME,
                        log_base
                    );
                }
                assert!(digits
                    .iter()
                    .all(|digit| digit.significant_bits() <= log_base));
                let mut limbs = Vec::new();
                if digits.is_empty() {
                } else if let Some(log_log_base) = log_base.checked_log_two() {
                    match log_log_base.cmp(&Limb::LOG_WIDTH) {
                        Ordering::Equal => {
                            limbs.extend(digits.iter().cloned().map(Limb::wrapping_from))
                        }
                        Ordering::Less => limbs.extend(
                            digits
                                .chunks(usize::wrapping_from(Limb::WIDTH >> log_log_base))
                                .map(|limb_digits| {
                                    Limb::from_power_of_two_digits_asc(log_base, limb_digits)
                                }),
                        ),
                        Ordering::Greater => {
                            for mut digit in digits.iter().cloned() {
                                let mask = $t::MAX.mod_power_of_two(Limb::WIDTH);
                                for _ in 0..u64::power_of_two(log_log_base - Limb::LOG_WIDTH) {
                                    limbs.push(Limb::wrapping_from(digit & mask));
                                    digit >>= Limb::WIDTH;
                                }
                            }
                        }
                    }
                } else {
                    reformat_slice(&mut limbs, digits.iter(), Limb::WIDTH, log_base);
                }
                Natural::from_owned_limbs_asc(limbs)
            }

            /// Converts a slice of digits into a `Natural`, where the base is a power of two. The
            /// base-2 logarithm of the base is specified. The input digits are in descending order:
            /// most- to least-significant. The type of each digit is `$t`, and `log_base` must be
            /// no larger than the width of `$t`.
            ///
            /// Time: worst case O(n)
            ///
            /// Additional memory: worst case O(n)
            ///
            /// where n = `digits.len()`
            ///
            /// # Panics
            /// Panics if `log_base` is greater than the width of `$t`, if `log_base` is zero, or if
            /// some digit is greater than 2<sup>`log_base`.</sup>
            ///
            /// # Examples
            /// ```
            /// extern crate malachite_base;
            /// extern crate malachite_nz;
            ///
            /// use malachite_base::num::logic::traits::PowerOfTwoDigits;
            /// use malachite_nz::natural::Natural;
            ///
            /// let digits: &[u64] = &[0, 0, 0];
            /// assert_eq!(Natural::from_power_of_two_digits_desc(6, digits), 0);
            ///
            /// let digits: &[u64] = &[0, 2];
            /// assert_eq!(Natural::from_power_of_two_digits_desc(6, digits), 2);
            ///
            /// let digits: &[u16] = &[1, 7, 3];
            /// assert_eq!(Natural::from_power_of_two_digits_desc(3, digits), 123);
            /// ```
            #[allow(exceeding_bitshifts)]
            fn from_power_of_two_digits_desc(log_base: u64, digits: &[$t]) -> Natural {
                assert_ne!(log_base, 0);
                if log_base > $t::WIDTH {
                    panic!(
                        "type {:?} is too small for a digit of width {}",
                        $t::NAME,
                        log_base
                    );
                }
                assert!(digits
                    .iter()
                    .all(|digit| digit.significant_bits() <= log_base));
                let mut limbs = Vec::new();
                if digits.is_empty() {
                } else if let Some(log_log_base) = log_base.checked_log_two() {
                    match log_log_base.cmp(&Limb::LOG_WIDTH) {
                        Ordering::Equal => {
                            limbs.extend(digits.iter().rev().cloned().map(Limb::wrapping_from))
                        }
                        Ordering::Less => limbs.extend(
                            digits
                                .rchunks(usize::wrapping_from(Limb::WIDTH >> log_log_base))
                                .map(|limb_digits| {
                                    Limb::from_power_of_two_digits_desc(log_base, limb_digits)
                                }),
                        ),
                        Ordering::Greater => {
                            for mut digit in digits.iter().rev().cloned() {
                                let mask = $t::MAX.mod_power_of_two(Limb::WIDTH);
                                for _ in 0..u64::power_of_two(log_log_base - Limb::LOG_WIDTH) {
                                    limbs.push(Limb::wrapping_from(digit & mask));
                                    digit >>= Limb::WIDTH;
                                }
                            }
                        }
                    }
                } else {
                    reformat_slice(&mut limbs, digits.iter().rev(), Limb::WIDTH, log_base);
                }
                Natural::from_owned_limbs_asc(limbs)
            }
        }

        impl Natural {
            pub fn $to_power_of_two_digits_asc_naive(&self, log_base: u64) -> Vec<$t> {
                assert_ne!(log_base, 0);
                if log_base > $t::WIDTH {
                    panic!(
                        "type {:?} is too small for a digit of width {}",
                        $t::NAME,
                        log_base
                    );
                }
                let digit_len = self
                    .significant_bits()
                    .div_round(log_base, RoundingMode::Ceiling);
                let mut digits = Vec::with_capacity(usize::exact_from(digit_len));
                let mut previous_index = 0;
                for _ in 0..digit_len {
                    let index = previous_index + log_base;
                    digits.push($t::exact_from(self.get_bits(previous_index, index)));
                    previous_index = index;
                }
                digits
            }

            pub fn $from_power_of_two_digits_asc_naive(log_base: u64, digits: &[$t]) -> Natural {
                assert_ne!(log_base, 0);
                if log_base > $t::WIDTH {
                    panic!(
                        "type {:?} is too small for a digit of width {}",
                        $t::NAME,
                        log_base
                    );
                }
                let mut n = Natural::ZERO;
                let mut previous_index = 0;
                for &digit in digits {
                    let index = previous_index + log_base;
                    n.assign_bits(previous_index, index, &Natural::from(digit));
                    previous_index = index;
                }
                n
            }
        }
    };
}

power_of_two_digits_primitive!(
    u8,
    _to_power_of_two_digits_asc_u8_naive,
    _from_power_of_two_digits_asc_u8_naive
);
power_of_two_digits_primitive!(
    u16,
    _to_power_of_two_digits_asc_u16_naive,
    _from_power_of_two_digits_asc_u16_naive
);
power_of_two_digits_primitive!(
    u32,
    _to_power_of_two_digits_asc_u32_naive,
    _from_power_of_two_digits_asc_u32_naive
);
power_of_two_digits_primitive!(
    u64,
    _to_power_of_two_digits_asc_u64_naive,
    _from_power_of_two_digits_asc_u64_naive
);
power_of_two_digits_primitive!(
    u128,
    _to_power_of_two_digits_asc_u128_naive,
    _from_power_of_two_digits_asc_u128_naive
);
power_of_two_digits_primitive!(
    usize,
    _to_power_of_two_digits_asc_usize_naive,
    _from_power_of_two_digits_asc_usize_naive
);

impl PowerOfTwoDigits<Natural> for Natural {
    /// Returns a `Vec` containing the digits of `self` in ascending order: least- to most-
    /// significant, where the base is a power of two. The base-2 logarithm of the base is
    /// specified. The type of each digit is `Natural`. If `self` is 0, the `Vec` is empty;
    /// otherwise, it ends with a nonzero digit.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Panics
    /// Panics if `log_base` is zero.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::basic::traits::{Two, Zero};
    /// use malachite_base::num::logic::traits::PowerOfTwoDigits;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     format!(
    ///         "{:?}",
    ///         PowerOfTwoDigits::<Natural>::to_power_of_two_digits_asc(&Natural::ZERO, 6)
    ///     ),
    ///     "[]"
    /// );
    /// assert_eq!(
    ///     format!(
    ///         "{:?}",
    ///         PowerOfTwoDigits::<Natural>::to_power_of_two_digits_asc(&Natural::TWO, 6)
    ///     ),
    ///     "[2]"
    /// );
    /// // 123_10 = 173_8
    /// assert_eq!(
    ///     format!(
    ///         "{:?}",
    ///         PowerOfTwoDigits::<Natural>::to_power_of_two_digits_asc(&Natural::from(123u32), 3)
    ///     ),
    ///     "[3, 7, 1]"
    /// );
    /// ```
    fn to_power_of_two_digits_asc(&self, log_base: u64) -> Vec<Natural> {
        assert_ne!(log_base, 0);
        if log_base <= Limb::WIDTH || self.limb_count() < 2 {
            return PowerOfTwoDigits::<Limb>::to_power_of_two_digits_asc(
                self,
                min(log_base, Limb::WIDTH),
            )
            .iter()
            .cloned()
            .map(Natural::from)
            .collect();
        }
        let limbs = match *self {
            Natural(Large(ref limbs)) => limbs,
            _ => unreachable!(),
        };
        let mut digits = Vec::new();
        if let Some(log_log_base) = log_base.checked_log_two() {
            assert!(log_log_base > Limb::LOG_WIDTH);
            digits.extend(
                limbs
                    .chunks(usize::power_of_two(log_log_base - Limb::LOG_WIDTH))
                    .map(Natural::from_limbs_asc),
            );
        } else {
            let mut digit = Natural::ZERO;
            let mut remaining_digit_bits = log_base;
            for &limb in limbs {
                let mut limb = limb;
                let mut remaining_limb_bits = Limb::WIDTH;
                while remaining_limb_bits != 0 {
                    let digit_index = log_base - remaining_digit_bits;
                    if remaining_limb_bits <= remaining_digit_bits {
                        digit.assign_bits(
                            digit_index,
                            digit_index + remaining_limb_bits,
                            &Natural::from(limb),
                        );
                        remaining_digit_bits -= remaining_limb_bits;
                        remaining_limb_bits = 0;
                    } else {
                        digit.assign_bits(digit_index, log_base, &Natural::from(limb));
                        limb >>= remaining_digit_bits;
                        remaining_limb_bits -= remaining_digit_bits;
                        remaining_digit_bits = 0;
                    }
                    if remaining_digit_bits == 0 {
                        digits.push(digit);
                        digit = Natural::ZERO;
                        remaining_digit_bits = log_base;
                    }
                }
            }
            if digit != 0 {
                digits.push(digit);
            }
        }
        digits.truncate(digits.len() - slice_trailing_zeros(&digits));
        digits
    }

    /// Returns a `Vec` containing the digits of `self` in descending order: most- to least-
    /// significant, where the base is a power of two. The base-2 logarithm of the base is
    /// specified. The type of each digit is `Natural`. If `self` is 0, the `Vec` is empty;
    /// otherwise, it begins with a nonzero digit.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Panics
    /// Panics if `log_base` is zero.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::basic::traits::{Two, Zero};
    /// use malachite_base::num::logic::traits::PowerOfTwoDigits;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     format!(
    ///         "{:?}",
    ///         PowerOfTwoDigits::<Natural>::to_power_of_two_digits_desc(&Natural::ZERO, 6)
    ///     ),
    ///     "[]"
    /// );
    /// assert_eq!(
    ///     format!(
    ///         "{:?}",
    ///         PowerOfTwoDigits::<Natural>::to_power_of_two_digits_desc(&Natural::TWO, 6)
    ///     ),
    ///     "[2]"
    /// );
    /// // 123_10 = 173_8
    /// assert_eq!(
    ///     format!(
    ///         "{:?}",
    ///         PowerOfTwoDigits::<Natural>::to_power_of_two_digits_desc(&Natural::from(123u32), 3)
    ///     ),
    ///     "[1, 7, 3]"
    /// );
    /// ```
    fn to_power_of_two_digits_desc(&self, log_base: u64) -> Vec<Natural> {
        let mut digits = self.to_power_of_two_digits_asc(log_base);
        digits.reverse();
        digits
    }

    /// Converts a slice of digits into a `Natural`, where the base is a power of two. The base-2
    /// logarithm of the base is specified. The input digits are in ascending order: least- to most-
    /// significant. The type of each digit is `Natural`.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// where n = `digits.len()` * `log_base`
    ///
    /// # Panics
    /// Panics if `log_base` is zero or if some digit is greater than 2<sup>`log_base`.</sup>
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::basic::traits::{One, Two, Zero};
    /// use malachite_base::num::logic::traits::PowerOfTwoDigits;
    /// use malachite_nz::natural::Natural;
    ///
    /// let digits = &[Natural::ZERO, Natural::ZERO, Natural::ZERO];
    /// assert_eq!(Natural::from_power_of_two_digits_asc(6, digits), 0);
    ///
    /// let digits = &[Natural::TWO, Natural::ZERO];
    /// assert_eq!(Natural::from_power_of_two_digits_asc(6, digits), 2);
    ///
    /// let digits = &[Natural::from(3u32), Natural::from(7u32), Natural::ONE];
    /// assert_eq!(Natural::from_power_of_two_digits_asc(3, digits), 123);
    /// ```
    fn from_power_of_two_digits_asc(log_base: u64, digits: &[Natural]) -> Natural {
        assert_ne!(log_base, 0);
        assert!(digits
            .iter()
            .all(|digit| digit.significant_bits() <= log_base));
        if digits.is_empty() {
            Natural::ZERO
        } else if let Some(log_log_base) = log_base.checked_log_two() {
            let mut limbs = Vec::new();
            match log_log_base.cmp(&Limb::LOG_WIDTH) {
                Ordering::Equal => limbs.extend(digits.iter().cloned().map(Limb::wrapping_from)),
                Ordering::Less => {
                    for chunk in digits.chunks(usize::wrapping_from(Limb::WIDTH >> log_log_base)) {
                        let mut limb = 0;
                        let mut offset = 0;
                        for digit in chunk {
                            limb |= Limb::wrapping_from(digit) << offset;
                            offset += log_base;
                        }
                        limbs.push(limb);
                    }
                }
                Ordering::Greater => {
                    let mut offset = 0;
                    let chunk_size = usize::wrapping_from(log_base >> Limb::LOG_WIDTH);
                    for digit in digits {
                        offset += chunk_size;
                        for limb in digit.limbs() {
                            limbs.push(limb);
                        }
                        limbs.resize(offset, 0);
                    }
                }
            }
            Natural::from_owned_limbs_asc(limbs)
        } else {
            let mut n = Natural::ZERO;
            let mut previous_index = 0;
            for digit in digits {
                let index = previous_index + log_base;
                n.assign_bits(previous_index, index, digit);
                previous_index = index;
            }
            n
        }
    }

    /// Converts a slice of digits into a `Natural`, where the base is a power of two. The base-2
    /// logarithm of the base is specified. The input digits are in descending order: least- to
    /// most-significant. The type of each digit is `Natural`.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// where n = `digits.len()` * `log_base`
    ///
    /// # Panics
    /// Panics if `log_base` is zero or if some digit is greater than 2<sup>`log_base`.</sup>
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::basic::traits::{One, Two, Zero};
    /// use malachite_base::num::logic::traits::PowerOfTwoDigits;
    /// use malachite_nz::natural::Natural;
    ///
    /// let digits = &[Natural::ZERO, Natural::ZERO, Natural::ZERO];
    /// assert_eq!(Natural::from_power_of_two_digits_desc(6, digits), 0);
    ///
    /// let digits = &[Natural::ZERO, Natural::TWO];
    /// assert_eq!(Natural::from_power_of_two_digits_desc(6, digits), 2);
    ///
    /// let digits = &[Natural::ONE, Natural::from(7u32), Natural::from(3u32)];
    /// assert_eq!(Natural::from_power_of_two_digits_desc(3, digits), 123);
    /// ```
    fn from_power_of_two_digits_desc(log_base: u64, digits: &[Natural]) -> Natural {
        assert_ne!(log_base, 0);
        assert!(digits
            .iter()
            .all(|digit| digit.significant_bits() <= log_base));
        if digits.is_empty() {
            Natural::ZERO
        } else if let Some(log_log_base) = log_base.checked_log_two() {
            let mut limbs = Vec::new();
            match log_log_base.cmp(&Limb::LOG_WIDTH) {
                Ordering::Equal => {
                    limbs.extend(digits.iter().rev().cloned().map(Limb::wrapping_from))
                }
                Ordering::Less => {
                    for chunk in digits.rchunks(usize::wrapping_from(Limb::WIDTH >> log_log_base)) {
                        let mut limb = 0;
                        let mut offset = 0;
                        for digit in chunk.iter().rev() {
                            limb |= Limb::wrapping_from(digit) << offset;
                            offset += log_base;
                        }
                        limbs.push(limb);
                    }
                }
                Ordering::Greater => {
                    let mut offset = 0;
                    let chunk_size = usize::wrapping_from(log_base >> Limb::LOG_WIDTH);
                    for digit in digits.iter().rev() {
                        offset += chunk_size;
                        for limb in digit.limbs() {
                            limbs.push(limb);
                        }
                        limbs.resize(offset, 0);
                    }
                }
            }
            Natural::from_owned_limbs_asc(limbs)
        } else {
            let mut n = Natural::ZERO;
            let mut previous_index = 0;
            for digit in digits.iter().rev() {
                let index = previous_index + log_base;
                n.assign_bits(previous_index, index, digit);
                previous_index = index;
            }
            n
        }
    }
}

impl Natural {
    pub fn _to_power_of_two_digits_asc_natural_naive(&self, log_base: u64) -> Vec<Natural> {
        assert_ne!(log_base, 0);
        let digit_len = self
            .significant_bits()
            .div_round(log_base, RoundingMode::Ceiling);
        let mut digits = Vec::with_capacity(usize::exact_from(digit_len));
        let mut previous_index = 0;
        for _ in 0..digit_len {
            let index = previous_index + log_base;
            digits.push(self.get_bits(previous_index, index));
            previous_index = index;
        }
        digits
    }

    pub fn _from_power_of_two_digits_asc_natural_naive(
        log_base: u64,
        digits: &[Natural],
    ) -> Natural {
        assert_ne!(log_base, 0);
        let mut n = Natural::ZERO;
        let mut previous_index = 0;
        for digit in digits {
            let index = previous_index + log_base;
            n.assign_bits(previous_index, index, digit);
            previous_index = index;
        }
        n
    }
}