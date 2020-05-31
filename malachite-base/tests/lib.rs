#![warn(
    clippy::cast_lossless,
    clippy::decimal_literal_representation,
    clippy::explicit_into_iter_loop,
    clippy::explicit_iter_loop,
    clippy::filter_map,
    clippy::filter_map_next,
    clippy::find_map,
    clippy::large_digit_groups,
    clippy::map_flatten,
    clippy::match_same_arms,
    clippy::missing_const_for_fn,
    clippy::mut_mut,
    clippy::needless_borrow,
    clippy::needless_continue,
    clippy::needless_pass_by_value,
    clippy::non_ascii_literal,
    clippy::option_map_unwrap_or,
    clippy::option_map_unwrap_or_else,
    clippy::print_stdout,
    clippy::redundant_closure_for_method_calls,
    clippy::result_map_unwrap_or_else,
    clippy::single_match_else,
    clippy::type_repetition_in_bounds,
    clippy::unused_self
)]
#![allow(clippy::cognitive_complexity, clippy::too_many_arguments)]

#[macro_use]
extern crate malachite_base;
extern crate malachite_base_test_util;

pub mod bools {
    pub mod constants;
    pub mod crement;
    pub mod not_assign;
}
pub mod comparison {
    pub mod macros;
}
pub mod chars;
pub mod named;
pub mod num {
    pub mod arithmetic {
        pub mod abs;
        pub mod add_mul;
        pub mod arithmetic_checked_shl;
        pub mod arithmetic_checked_shr;
        pub mod checked_abs;
        pub mod checked_add_mul;
        pub mod checked_neg;
        pub mod checked_sub_mul;
        pub mod div_mod;
        pub mod divisible_by_power_of_two;
        pub mod eq_mod_power_of_two;
        pub mod log_two;
        pub mod mod_add;
        pub mod mod_is_reduced;
        pub mod mod_mul;
        pub mod mod_neg;
        pub mod mod_power_of_two;
        pub mod mod_power_of_two_add;
        pub mod mod_power_of_two_is_reduced;
        pub mod mod_power_of_two_mul;
        pub mod mod_power_of_two_neg;
        pub mod mod_power_of_two_shl;
        pub mod mod_power_of_two_shr;
        pub mod mod_power_of_two_sub;
        pub mod mod_sub;
        pub mod neg;
        pub mod overflowing_abs;
        pub mod overflowing_add;
        pub mod overflowing_add_mul;
        pub mod overflowing_div;
        pub mod overflowing_mul;
        pub mod overflowing_neg;
        pub mod overflowing_sub;
        pub mod overflowing_sub_mul;
        pub mod parity;
        pub mod power_of_two;
        pub mod round_to_multiple_of_power_of_two;
        pub mod saturating_abs;
        pub mod saturating_add;
        pub mod saturating_add_mul;
        pub mod saturating_mul;
        pub mod saturating_neg;
        pub mod saturating_sub;
        pub mod saturating_sub_mul;
        pub mod shl_round;
        pub mod shr_round;
        pub mod sign;
        pub mod sub_mul;
        pub mod wrapping_abs;
        pub mod wrapping_add;
        pub mod wrapping_add_mul;
        pub mod wrapping_div;
        pub mod wrapping_mul;
        pub mod wrapping_neg;
        pub mod wrapping_sub;
        pub mod wrapping_sub_mul;
        pub mod x_mul_y_is_zz;
        pub mod xx_add_yy_is_zz;
        pub mod xx_div_mod_y_is_qr;
        pub mod xx_sub_yy_is_zz;
        pub mod xxx_add_yyy_is_zzz;
        pub mod xxx_sub_yyy_is_zzz;
        pub mod xxxx_add_yyyy_is_zzzz;
    }
    pub mod basic {
        pub mod constants;
        pub mod crement;
        pub mod iverson;
    }
    pub mod comparison;
    pub mod conversion;
    pub mod logic {
        pub mod bit_access;
        pub mod bit_block_access;
        pub mod bit_convertible;
        pub mod bit_iterable;
        pub mod bit_scan;
        pub mod get_highest_bit;
        pub mod hamming_distance;
        pub mod low_mask;
        pub mod not;
        pub mod power_of_two_digit_iterable;
        pub mod power_of_two_digits;
        pub mod rotate;
        pub mod significant_bits;
    }
}
pub mod round;
pub mod slices;
pub mod strings;
pub mod vecs;
