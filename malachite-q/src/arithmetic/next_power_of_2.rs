use crate::Rational;
use malachite_base::num::arithmetic::traits::{NextPowerOf2, NextPowerOf2Assign, PowerOf2};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use std::cmp::Ordering;

impl NextPowerOf2 for Rational {
    type Output = Rational;

    /// Finds the smallest power of 2 greater than or equal to a [`Rational`]. The [`Rational`] is
    /// taken by value.
    ///
    /// $f(x) = 2^{\lceil \log_2 x \rceil}$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` is less than or equal to zero.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    ///
    /// use malachite_base::num::arithmetic::traits::NextPowerOf2;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(Rational::from(123).next_power_of_2(), 128);
    /// assert_eq!(Rational::from_signeds(1, 10).next_power_of_2().to_string(), "1/8");
    /// ```
    #[inline]
    fn next_power_of_2(self) -> Rational {
        assert!(self > 0);
        let mut exponent = i64::exact_from(self.numerator.significant_bits())
            - i64::exact_from(self.denominator.significant_bits());
        match self.numerator.cmp_normalized(&self.denominator) {
            Ordering::Equal => return self,
            Ordering::Greater => exponent += 1,
            _ => {}
        }
        Rational::power_of_2(exponent)
    }
}

impl<'a> NextPowerOf2 for &'a Rational {
    type Output = Rational;

    /// Finds the smallest power of 2 greater than or equal to a [`Rational`]. The [`Rational`] is
    /// taken by reference.
    ///
    /// $f(x) = 2^{\lceil \log_2 x \rceil}$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` is less than or equal to zero.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    ///
    /// use malachite_base::num::arithmetic::traits::NextPowerOf2;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!((&Rational::from(123)).next_power_of_2(), 128);
    /// assert_eq!((&Rational::from_signeds(1, 10)).next_power_of_2().to_string(), "1/8");
    /// ```
    fn next_power_of_2(self) -> Rational {
        assert!(*self > 0);
        let mut exponent = i64::exact_from(self.numerator.significant_bits())
            - i64::exact_from(self.denominator.significant_bits());
        if self.numerator.cmp_normalized(&self.denominator) == Ordering::Greater {
            exponent += 1;
        }
        Rational::power_of_2(exponent)
    }
}

impl NextPowerOf2Assign for Rational {
    /// Finds the smallest power of 2 greater than or equal to a [`Rational`]. The [`Rational`] is
    /// taken by reference.
    ///
    /// $f(x) = 2^{\lceil \log_2 x \rceil}$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` is less than or equal to zero.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    ///
    /// use malachite_base::num::arithmetic::traits::NextPowerOf2Assign;
    /// use malachite_q::Rational;
    /// use std::str::FromStr;
    ///
    /// let mut x = Rational::from(123);
    /// x.next_power_of_2_assign();
    /// assert_eq!(x, 128);
    ///
    /// let mut x = Rational::from_signeds(1, 10);
    /// x.next_power_of_2_assign();
    /// assert_eq!(x.to_string(), "1/8");
    /// ```
    #[inline]
    fn next_power_of_2_assign(&mut self) {
        *self = (&*self).next_power_of_2();
    }
}
