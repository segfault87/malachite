use natural::Natural::{self, Large, Small};

impl Natural {
    /// Returns the number of limbs, or base-2^(32) digits, of `self`. Zero has 0 limbs.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Example
    /// ```
    /// use malachite_native::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(Natural::from(0u32).limb_count(), 0);
    /// assert_eq!(Natural::from(123u32).limb_count(), 1);
    /// assert_eq!(Natural::from_str("1000000000000").unwrap().limb_count(), 2);
    /// ```
    pub fn limb_count(&self) -> u64 {
        match *self {
            Small(0) => 0,
            Small(_) => 1,
            Large(ref limbs) => limbs.len() as u64,
        }
    }
}
