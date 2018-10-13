use integer::Integer;
use malachite_base::limbs::limbs_leading_zero_limbs;
use malachite_base::num::{BitScan, PrimitiveInteger};
use natural::logic::bit_scan::{limbs_index_of_next_false_bit, limbs_index_of_next_true_bit};
use natural::Natural::{self, Large, Small};
use std::u32;

/// Interpreting a slice of `u32`s as the limbs (in ascending order) of the negative of an
/// `Integer`, finds the lowest index greater than or equal to `starting_index` at which the
/// `Integer` has a `false` bit. If the starting index is too large and there are no more `false`
/// bits above it, `None` is returned.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `limbs.len()`
///
/// # Example
/// ```
/// use malachite_nz::integer::logic::bit_scan::limbs_index_of_next_false_bit_neg;
///
/// assert_eq!(limbs_index_of_next_false_bit_neg(&[0, 0b101], 0), Some(0));
/// assert_eq!(limbs_index_of_next_false_bit_neg(&[0, 0b101], 20), Some(20));
/// assert_eq!(limbs_index_of_next_false_bit_neg(&[0, 0b101], 31), Some(31));
/// assert_eq!(limbs_index_of_next_false_bit_neg(&[0, 0b101], 32), Some(34));
/// assert_eq!(limbs_index_of_next_false_bit_neg(&[0, 0b101], 33), Some(34));
/// assert_eq!(limbs_index_of_next_false_bit_neg(&[0, 0b101], 34), Some(34));
/// assert_eq!(limbs_index_of_next_false_bit_neg(&[0, 0b101], 35), None);
/// assert_eq!(limbs_index_of_next_false_bit_neg(&[0, 0b101], 100), None);
/// ```
pub fn limbs_index_of_next_false_bit_neg(limbs: &[u32], mut starting_index: u64) -> Option<u64> {
    let n = limbs.len();
    let i = limbs_leading_zero_limbs(limbs);
    assert!(i < n);
    let starting_limb_index = (starting_index >> u32::LOG_WIDTH) as usize;
    if starting_limb_index >= n {
        return None;
    }
    let after_boundary_offset = ((i as u64) + 1) << u32::LOG_WIDTH;
    if starting_limb_index < i {
        return Some(starting_index);
    } else if starting_limb_index == i {
        let within_limb_index = starting_index & u64::from(u32::WIDTH_MASK);
        if let Some(result) = limbs[i]
            .wrapping_neg()
            .index_of_next_false_bit(within_limb_index)
        {
            if result < u32::WIDTH.into() {
                return Some(((i as u64) << u32::LOG_WIDTH) + result);
            } else {
                starting_index = 0;
            }
        }
    } else {
        starting_index -= after_boundary_offset;
    }
    limbs_index_of_next_true_bit(&limbs[i + 1..], starting_index)
        .map(|result| result + after_boundary_offset)
}

/// Interpreting a slice of `u32`s as the limbs (in ascending order) of the negative of an
/// `Integer`, finds the lowest index greater than or equal to `starting_index` at which the
/// `Integer` has a `true` bit.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `limbs.len()`
///
/// # Example
/// ```
/// use malachite_nz::integer::logic::bit_scan::limbs_index_of_next_true_bit_neg;
///
/// assert_eq!(limbs_index_of_next_true_bit_neg(&[0, 0b101], 0), 32);
/// assert_eq!(limbs_index_of_next_true_bit_neg(&[0, 0b101], 20), 32);
/// assert_eq!(limbs_index_of_next_true_bit_neg(&[0, 0b101], 31), 32);
/// assert_eq!(limbs_index_of_next_true_bit_neg(&[0, 0b101], 32), 32);
/// assert_eq!(limbs_index_of_next_true_bit_neg(&[0, 0b101], 33), 33);
/// assert_eq!(limbs_index_of_next_true_bit_neg(&[0, 0b101], 34), 35);
/// assert_eq!(limbs_index_of_next_true_bit_neg(&[0, 0b101], 35), 35);
/// assert_eq!(limbs_index_of_next_true_bit_neg(&[0, 0b101], 36), 36);
/// assert_eq!(limbs_index_of_next_true_bit_neg(&[0, 0b101], 100), 100);
/// ```
pub fn limbs_index_of_next_true_bit_neg(limbs: &[u32], mut starting_index: u64) -> u64 {
    let n = limbs.len();
    let i = limbs_leading_zero_limbs(limbs);
    assert!(i < n);
    let mut starting_limb_index = (starting_index >> u32::LOG_WIDTH) as usize;
    if starting_limb_index >= n {
        return starting_index;
    }
    let after_boundary_offset = ((i as u64) + 1) << u32::LOG_WIDTH;
    if starting_limb_index < i {
        starting_index = (i as u64) << u32::LOG_WIDTH;
        starting_limb_index = i;
    }
    if starting_limb_index == i {
        let within_limb_index = starting_index & u64::from(u32::WIDTH_MASK);
        if let Some(result) = limbs[i]
            .wrapping_neg()
            .index_of_next_true_bit(within_limb_index)
        {
            return ((i as u64) << u32::LOG_WIDTH) + result;
        } else {
            starting_index = 0;
        }
    } else {
        starting_index -= after_boundary_offset;
    }
    limbs_index_of_next_false_bit(&limbs[i + 1..], starting_index) + after_boundary_offset
}

impl<'a> BitScan for &'a Integer {
    /// Finds the lowest index greater than or equal to `starting_index` at which the `Integer` has
    /// a `false` bit. If the `Integer` as negative, and the starting index is too large and there
    /// are no more `false` bits above it, `None` is returned.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Example
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::BitScan;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     assert_eq!((-Integer::from(0x5_0000_0000u64)).index_of_next_false_bit(0), Some(0));
    ///     assert_eq!((-Integer::from(0x5_0000_0000u64)).index_of_next_false_bit(20), Some(20));
    ///     assert_eq!((-Integer::from(0x5_0000_0000u64)).index_of_next_false_bit(31), Some(31));
    ///     assert_eq!((-Integer::from(0x5_0000_0000u64)).index_of_next_false_bit(32), Some(34));
    ///     assert_eq!((-Integer::from(0x5_0000_0000u64)).index_of_next_false_bit(33), Some(34));
    ///     assert_eq!((-Integer::from(0x5_0000_0000u64)).index_of_next_false_bit(34), Some(34));
    ///     assert_eq!((-Integer::from(0x5_0000_0000u64)).index_of_next_false_bit(35), None);
    ///     assert_eq!((-Integer::from(0x5_0000_0000u64)).index_of_next_false_bit(100), None);
    /// }
    /// ```
    fn index_of_next_false_bit(self, starting_index: u64) -> Option<u64> {
        if self.sign {
            self.abs.index_of_next_false_bit(starting_index)
        } else {
            self.abs.index_of_next_false_bit_neg(starting_index)
        }
    }

    /// Finds the lowest index greater than or equal to `starting_index` at which the `Integer` has
    /// a `true` bit. If the `Integer` is non-negative, and the starting index is too large and
    /// there are no more `true` bits above it, `None` is returned.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Example
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::BitScan;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     assert_eq!((-Integer::from(0x5_0000_0000u64)).index_of_next_true_bit(0), Some(32));
    ///     assert_eq!((-Integer::from(0x5_0000_0000u64)).index_of_next_true_bit(20), Some(32));
    ///     assert_eq!((-Integer::from(0x5_0000_0000u64)).index_of_next_true_bit(31), Some(32));
    ///     assert_eq!((-Integer::from(0x5_0000_0000u64)).index_of_next_true_bit(32), Some(32));
    ///     assert_eq!((-Integer::from(0x5_0000_0000u64)).index_of_next_true_bit(33), Some(33));
    ///     assert_eq!((-Integer::from(0x5_0000_0000u64)).index_of_next_true_bit(34), Some(35));
    ///     assert_eq!((-Integer::from(0x5_0000_0000u64)).index_of_next_true_bit(35), Some(35));
    ///     assert_eq!((-Integer::from(0x5_0000_0000u64)).index_of_next_true_bit(36), Some(36));
    ///     assert_eq!((-Integer::from(0x5_0000_0000u64)).index_of_next_true_bit(100), Some(100));
    /// }
    /// ```
    fn index_of_next_true_bit(self, starting_index: u64) -> Option<u64> {
        if self.sign {
            self.abs.index_of_next_true_bit(starting_index)
        } else {
            Some(self.abs.index_of_next_true_bit_neg(starting_index))
        }
    }
}

impl Natural {
    // self != 0
    fn index_of_next_false_bit_neg(&self, starting_index: u64) -> Option<u64> {
        match *self {
            Small(small) => {
                if starting_index >= u64::from(u32::WIDTH) {
                    None
                } else {
                    let index = ((small - 1) & !((1 << starting_index) - 1))
                        .trailing_zeros()
                        .into();
                    if index == u32::WIDTH.into() {
                        None
                    } else {
                        Some(index)
                    }
                }
            }
            Large(ref limbs) => limbs_index_of_next_false_bit_neg(limbs, starting_index),
        }
    }

    // self != 0
    fn index_of_next_true_bit_neg(&self, starting_index: u64) -> u64 {
        match *self {
            Small(small) => {
                if starting_index >= u64::from(u32::WIDTH) {
                    starting_index
                } else {
                    (!((small - 1) | ((1 << starting_index) - 1)))
                        .trailing_zeros()
                        .into()
                }
            }
            Large(ref limbs) => limbs_index_of_next_true_bit_neg(limbs, starting_index),
        }
    }
}