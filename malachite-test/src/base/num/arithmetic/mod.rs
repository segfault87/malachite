use common::DemoBenchRegistry;

pub mod abs;
pub mod add_mul;
pub mod arithmetic_checked_shl;
pub mod arithmetic_checked_shr;
pub mod checked_add_mul;
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

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    abs::register(registry);
    add_mul::register(registry);
    arithmetic_checked_shl::register(registry);
    arithmetic_checked_shr::register(registry);
    checked_add_mul::register(registry);
    checked_sub_mul::register(registry);
    div_mod::register(registry);
    divisible_by_power_of_two::register(registry);
    eq_mod_power_of_two::register(registry);
    log_two::register(registry);
    mod_add::register(registry);
    mod_is_reduced::register(registry);
    mod_mul::register(registry);
    mod_neg::register(registry);
    mod_power_of_two::register(registry);
    mod_power_of_two_add::register(registry);
    mod_power_of_two_is_reduced::register(registry);
    mod_power_of_two_mul::register(registry);
    mod_power_of_two_neg::register(registry);
    mod_power_of_two_shl::register(registry);
    mod_power_of_two_shr::register(registry);
    mod_power_of_two_sub::register(registry);
    mod_sub::register(registry);
    neg::register(registry);
    overflowing_abs::register(registry);
    overflowing_add::register(registry);
    overflowing_add_mul::register(registry);
    overflowing_div::register(registry);
    overflowing_mul::register(registry);
    overflowing_neg::register(registry);
    overflowing_sub::register(registry);
    overflowing_sub_mul::register(registry);
    parity::register(registry);
    power_of_two::register(registry);
    round_to_multiple_of_power_of_two::register(registry);
    saturating_abs::register(registry);
    saturating_add::register(registry);
    saturating_add_mul::register(registry);
    saturating_mul::register(registry);
    saturating_neg::register(registry);
    saturating_sub::register(registry);
    saturating_sub_mul::register(registry);
    shl_round::register(registry);
    shr_round::register(registry);
    sign::register(registry);
    sub_mul::register(registry);
    wrapping_abs::register(registry);
    wrapping_add::register(registry);
    wrapping_add_mul::register(registry);
    wrapping_div::register(registry);
    wrapping_mul::register(registry);
    wrapping_neg::register(registry);
    wrapping_sub::register(registry);
    wrapping_sub_mul::register(registry);
    x_mul_y_is_zz::register(registry);
    xx_add_yy_is_zz::register(registry);
    xx_div_mod_y_is_qr::register(registry);
    xx_sub_yy_is_zz::register(registry);
    xxx_add_yyy_is_zzz::register(registry);
    xxx_sub_yyy_is_zzz::register(registry);
    xxxx_add_yyyy_is_zzzz::register(registry);
}
