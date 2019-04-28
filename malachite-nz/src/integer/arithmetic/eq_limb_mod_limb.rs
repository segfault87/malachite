use malachite_base::num::traits::{EqMod, NegMod};

use integer::Integer;
use natural::arithmetic::eq_limb_mod_limb::limbs_eq_limb_mod_limb;
use natural::Natural::{self, Large, Small};
use platform::Limb;

/// Interpreting a slice of `Limb`s as the limbs of a `Natural` in ascending order, determines
/// whether that `Natural` is equal to the negative of a limb mod a given `Limb` modulus.
///
/// This function assumes that `modulus` is nonzero, `limbs` has at least two elements, and the last
/// element of `limbs` is nonzero.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `limbs.len()`
///
/// # Panics
/// Panics if the length of `limbs` is less than 2.
///
/// # Example
/// ```
/// use malachite_nz::integer::arithmetic::eq_limb_mod_limb::limbs_eq_neg_limb_mod_limb;
///
/// assert_eq!(limbs_eq_neg_limb_mod_limb(&[6, 7], 3, 2), false);
/// assert_eq!(limbs_eq_neg_limb_mod_limb(&[100, 101, 102], 1_232, 10), true);
/// ```
pub fn limbs_eq_neg_limb_mod_limb(limbs: &[Limb], limb: Limb, modulus: Limb) -> bool {
    limbs_eq_limb_mod_limb(limbs, limb.neg_mod(modulus), modulus)
}

impl<'a> EqMod<Limb, Limb> for &'a Integer {
    /// Returns whether this `Integer` is equivalent to a `Limb` mod a `Limb` `modulus`; that is,
    /// whether `self` - other is a multiple of `modulus`. Two numbers are equal to each other mod 0
    /// iff they are equal.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::traits::EqMod;
    /// use malachite_nz::integer::Integer;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     assert_eq!(Integer::from(13u32).eq_mod(21, 8), true);
    ///     assert_eq!(Integer::from_str("987654321").unwrap().eq_mod(321u32, 1_000u32), true);
    ///     assert_eq!(Integer::from_str("987654321").unwrap().eq_mod(322u32, 1_000u32), false);
    ///     assert_eq!(Integer::from_str("-987654321").unwrap().eq_mod(679u32, 1_000u32), true);
    ///     assert_eq!(Integer::from_str("-987654321").unwrap().eq_mod(680u32, 1_000u32), false);
    /// }
    /// ```
    fn eq_mod(self, other: Limb, modulus: Limb) -> bool {
        if self.sign {
            self.abs.eq_mod(other, modulus)
        } else {
            self.abs.eq_neg_limb_mod_limb(other, modulus)
        }
    }
}

#[cfg(feature = "64_bit_limbs")]
impl<'a> EqMod<u32, u32> for &'a Integer {
    #[inline]
    fn eq_mod(self, other: u32, modulus: u32) -> bool {
        self.eq_mod(Limb::from(other), Limb::from(modulus))
    }
}

impl<'a> EqMod<&'a Integer, Limb> for Limb {
    /// Returns whether this `Limb` is equivalent to an `Integer` mod a `Limb` `modulus`; that is,
    /// whether `self` - other is a multiple of `modulus`. Two numbers are equal to each other mod 0
    /// iff they are equal.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// where n = `other.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::traits::EqMod;
    /// use malachite_nz::integer::Integer;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     assert_eq!(21u32.eq_mod(&Integer::from(13), 8u32), true);
    ///     assert_eq!(321u32.eq_mod(&Integer::from_str("987654321").unwrap(), 1_000u32), true);
    ///     assert_eq!(322u32.eq_mod(&Integer::from_str("987654321").unwrap(), 1_000u32), false);
    ///     assert_eq!(679u32.eq_mod(&Integer::from_str("-987654321").unwrap(), 1_000u32), true);
    ///     assert_eq!(680u32.eq_mod(&Integer::from_str("-987654321").unwrap(), 1_000u32), false);
    /// }
    /// ```
    #[inline]
    fn eq_mod(self, other: &'a Integer, modulus: Limb) -> bool {
        other.eq_mod(self, modulus)
    }
}

#[cfg(feature = "64_bit_limbs")]
impl<'a> EqMod<&'a Integer, u32> for u32 {
    #[inline]
    fn eq_mod(self, other: &'a Integer, modulus: u32) -> bool {
        Limb::from(self).eq_mod(other, Limb::from(modulus))
    }
}

impl<'a> EqMod<Limb, &'a Integer> for Limb {
    /// Returns whether this `Limb` is equivalent to a `Limb` mod an `Integer` `modulus`; that is,
    /// whether `self` - other is a multiple of `modulus`. Two numbers are equal to each other mod 0
    /// iff they are equal.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// where n = `other.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::traits::EqMod;
    /// use malachite_nz::integer::Integer;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     assert_eq!(21.eq_mod(8, &Integer::from(13)), true);
    ///     assert_eq!(21.eq_mod(8, &Integer::from(-13)), true);
    ///     assert_eq!(21.eq_mod(21, &Integer::from_str("12345678987654321").unwrap()), true);
    ///     assert_eq!(21.eq_mod(21, &Integer::from_str("-12345678987654321").unwrap()), true);
    ///     assert_eq!(21.eq_mod(22, &Integer::from_str("12345678987654321").unwrap()), false);
    ///     assert_eq!(21.eq_mod(22, &Integer::from_str("-12345678987654321").unwrap()), false);
    /// }
    /// ```
    #[inline]
    fn eq_mod(self, other: Limb, modulus: &'a Integer) -> bool {
        self.eq_mod(other, &modulus.abs)
    }
}

#[cfg(feature = "64_bit_limbs")]
impl<'a> EqMod<u32, &'a Integer> for u32 {
    #[inline]
    fn eq_mod(self, other: u32, modulus: &'a Integer) -> bool {
        Limb::from(self).eq_mod(Limb::from(other), modulus)
    }
}

impl Natural {
    // other cannot be zero.
    pub(crate) fn eq_neg_limb_mod_limb(&self, other: Limb, modulus: Limb) -> bool {
        modulus != 0
            && match *self {
                Small(small) => small % modulus == other.neg_mod(modulus),
                Large(ref limbs) => limbs_eq_neg_limb_mod_limb(limbs, other, modulus),
            }
    }
}
