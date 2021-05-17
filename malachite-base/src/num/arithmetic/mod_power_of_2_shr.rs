use num::arithmetic::traits::{
    ModPowerOf2Shl, ModPowerOf2ShlAssign, ModPowerOf2Shr, ModPowerOf2ShrAssign, UnsignedAbs,
};
use num::basic::integers::PrimitiveInt;
use num::basic::traits::Zero;
use num::conversion::traits::WrappingFrom;
use std::ops::{Shr, ShrAssign};

fn _mod_power_of_2_shr_signed<
    T: ModPowerOf2Shl<U, Output = T> + PrimitiveInt + Shr<U, Output = T>,
    U: Copy + Ord + WrappingFrom<u64> + Zero,
    S: Copy + Ord + UnsignedAbs<Output = U> + Zero,
>(
    x: T,
    other: S,
    pow: u64,
) -> T {
    assert!(pow <= T::WIDTH);
    let other_abs = other.unsigned_abs();
    if other >= S::ZERO {
        let width = U::wrapping_from(T::WIDTH);
        if width != U::ZERO && other_abs >= width {
            T::ZERO
        } else {
            x >> other_abs
        }
    } else {
        x.mod_power_of_2_shl(other_abs, pow)
    }
}

fn _mod_power_of_2_shr_assign_signed<
    T: ModPowerOf2ShlAssign<U> + PrimitiveInt + ShrAssign<U>,
    U: Copy + Ord + WrappingFrom<u64> + Zero,
    S: Copy + Ord + UnsignedAbs<Output = U> + Zero,
>(
    x: &mut T,
    other: S,
    pow: u64,
) {
    assert!(pow <= T::WIDTH);
    let other_abs = other.unsigned_abs();
    if other >= S::ZERO {
        let width = U::wrapping_from(T::WIDTH);
        if width != U::ZERO && other_abs >= width {
            *x = T::ZERO;
        } else {
            *x >>= other_abs;
        }
    } else {
        x.mod_power_of_2_shl_assign(other_abs, pow);
    }
}

macro_rules! impl_mod_power_of_2_shr_signed {
    ($t:ident) => {
        macro_rules! impl_mod_power_of_2_shr_signed_inner {
            ($u:ident) => {
                impl ModPowerOf2Shr<$u> for $t {
                    type Output = $t;

                    /// Computes `self >> other` mod 2<sup>`pow`</sup>. Assumes the input is already
                    /// reduced mod 2<sup>`pow`</sup>.
                    ///
                    /// Time: worst case O(1)
                    ///
                    /// Additional memory: worst case O(1)
                    ///
                    /// # Examples
                    /// ```
                    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Shr;
                    ///
                    /// assert_eq!(10u8.mod_power_of_2_shr(2i64, 4), 2);
                    /// assert_eq!(12u32.mod_power_of_2_shr(-2i8, 5), 16);
                    /// ```
                    #[inline]
                    fn mod_power_of_2_shr(self, other: $u, pow: u64) -> $t {
                        _mod_power_of_2_shr_signed(self, other, pow)
                    }
                }

                impl ModPowerOf2ShrAssign<$u> for $t {
                    /// Replaces `self` with `self >> other` mod 2<sup>`pow`</sup>. Assumes the
                    /// input is already reduced mod 2<sup>`pow`</sup>.
                    ///
                    /// Time: worst case O(1)
                    ///
                    /// Additional memory: worst case O(1)
                    ///
                    /// # Examples
                    /// ```
                    /// use malachite_base::num::arithmetic::traits::ModPowerOf2ShrAssign;
                    ///
                    /// let mut n = 10u8;
                    /// n.mod_power_of_2_shr_assign(2i64, 4);
                    /// assert_eq!(n, 2);
                    ///
                    /// let mut n = 12u32;
                    /// n.mod_power_of_2_shr_assign(-2i8, 5);
                    /// assert_eq!(n, 16);
                    /// ```
                    #[inline]
                    fn mod_power_of_2_shr_assign(&mut self, other: $u, pow: u64) {
                        _mod_power_of_2_shr_assign_signed(self, other, pow)
                    }
                }
            };
        }
        apply_to_signeds!(impl_mod_power_of_2_shr_signed_inner);
    };
}
apply_to_unsigneds!(impl_mod_power_of_2_shr_signed);