use integer::Integer;
use natural::Natural;
use std::cmp::Ordering;

/// Compares an `Integer` to a `Natural`.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = min(`self.significant_bits(), other.significant_bits()`)
///
/// # Examples
/// ```
/// use malachite_nz::integer::Integer;
/// use malachite_nz::natural::Natural;
///
/// assert!(Integer::from(123) > Natural::from(122u32));
/// assert!(Integer::from(123) >= Natural::from(122u32));
/// assert!(Integer::from(123) < Natural::from(124u32));
/// assert!(Integer::from(123) <= Natural::from(124u32));
/// assert!(Integer::from(-123) < Natural::from(123u32));
/// assert!(Integer::from(-123) <= Natural::from(123u32));
/// ```
impl PartialOrd<Natural> for Integer {
    fn partial_cmp(&self, other: &Natural) -> Option<Ordering> {
        if self.sign {
            self.abs.partial_cmp(other)
        } else {
            Some(Ordering::Less)
        }
    }
}

/// Compares a `Natural` to an `Integer`.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = min(`self.significant_bits(), other.significant_bits()`)
///
/// # Examples
/// ```
/// use malachite_nz::integer::Integer;
/// use malachite_nz::natural::Natural;
///
/// assert!(Natural::from(123u32) > Integer::from(122));
/// assert!(Natural::from(123u32) >= Integer::from(122));
/// assert!(Natural::from(123u32) < Integer::from(124));
/// assert!(Natural::from(123u32) <= Integer::from(124));
/// assert!(Natural::from(123u32) > Integer::from(-123));
/// assert!(Natural::from(123u32) >= Integer::from(-123));
/// ```
impl PartialOrd<Integer> for Natural {
    fn partial_cmp(&self, other: &Integer) -> Option<Ordering> {
        if other.sign {
            self.partial_cmp(&other.abs)
        } else {
            Some(Ordering::Greater)
        }
    }
}