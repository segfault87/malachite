use integer::Integer;
use malachite_base::misc::{CheckedFrom, WrappingFrom};
use std::i32;

impl<'a> CheckedFrom<&'a Integer> for i32 {
    /// an `i32`.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Example
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::misc::CheckedFrom;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     assert_eq!(format!("{:?}", i32::checked_from(&Integer::from(123))), "Some(123)");
    ///     assert_eq!(format!("{:?}", i32::checked_from(&Integer::from(-123))), "Some(-123)");
    ///     assert_eq!(format!("{:?}", i32::checked_from(&Integer::trillion())), "None");
    ///     assert_eq!(format!("{:?}", i32::checked_from(&(-Integer::trillion()))), "None");
    /// }
    /// ```
    fn checked_from(value: &Integer) -> Option<i32> {
        if *value >= i32::MIN && *value <= i32::MAX {
            Some(i32::wrapping_from(value))
        } else {
            None
        }
    }
}

impl<'a> WrappingFrom<&'a Integer> for i32 {
    /// Converts an `Integer` to a `i32`, wrapping mod 2<sup>32</sup>.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Example
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::misc::WrappingFrom;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     assert_eq!(i32::wrapping_from(&Integer::from(123)).to_string(), "123");
    ///     assert_eq!(i32::wrapping_from(&Integer::from(-123)).to_string(), "-123");
    ///     assert_eq!(i32::wrapping_from(&Integer::trillion()).to_string(), "-727379968");
    ///     assert_eq!(i32::wrapping_from(&(-Integer::trillion())).to_string(), "727379968");
    /// }
    /// ```
    fn wrapping_from(value: &Integer) -> i32 {
        u32::wrapping_from(value) as i32
    }
}