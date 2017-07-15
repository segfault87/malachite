use integer::Integer::{self, Small};
use traits::Assign;

/// Assigns an `i32` to an `Integer`.
///
/// # Examples
/// ```
/// use malachite_gmp::integer::Integer;
/// use malachite_gmp::traits::Assign;
///
/// let mut x = Integer::from(456);
/// x.assign(-123);
/// assert_eq!(x.to_string(), "-123");
/// ```
impl Assign<i32> for Integer {
    fn assign(&mut self, other: i32) {
        *self = Small(other);
    }
}
