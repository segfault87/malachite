use crate::Rational;
use malachite_nz::natural::Natural;

impl Rational {
    /// Extracts the numerator of a [`Rational`], taking the [`Rational`] by reference and cloning.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_q::Rational;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(Rational::from_str("2/3").unwrap().to_numerator(), 2);
    /// assert_eq!(Rational::from_str("0").unwrap().to_numerator(), 0);
    /// ```
    #[inline]
    pub fn to_numerator(&self) -> Natural {
        self.numerator.clone()
    }

    /// Extracts the denominator of a [`Rational`], taking the [`Rational`] by reference and
    /// cloning.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_q::Rational;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(Rational::from_str("2/3").unwrap().to_denominator(), 3);
    /// assert_eq!(Rational::from_str("0").unwrap().to_denominator(), 1);
    /// ```
    #[inline]
    pub fn to_denominator(&self) -> Natural {
        self.denominator.clone()
    }

    /// Extracts the numerator and denominator of a [`Rational`], taking the [`Rational`] by
    /// reference and cloning.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    ///
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_q::Rational;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(
    ///     Rational::from_str("2/3").unwrap().to_numerator_and_denominator().to_debug_string(),
    ///     "(2, 3)"
    /// );
    /// assert_eq!(
    ///     Rational::from_str("0").unwrap().to_numerator_and_denominator().to_debug_string(),
    ///     "(0, 1)"
    /// );
    /// ```
    #[inline]
    pub fn to_numerator_and_denominator(&self) -> (Natural, Natural) {
        (self.numerator.clone(), self.denominator.clone())
    }

    /// Extracts the numerator of a [`Rational`], taking the [`Rational`] by value.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_q::Rational;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(Rational::from_str("2/3").unwrap().into_numerator(), 2);
    /// assert_eq!(Rational::from_str("0").unwrap().into_numerator(), 0);
    /// ```
    #[inline]
    #[allow(clippy::missing_const_for_fn)]
    pub fn into_numerator(self) -> Natural {
        self.numerator
    }

    /// Extracts the denominator of a [`Rational`], taking the [`Rational`] by value.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_q::Rational;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(Rational::from_str("2/3").unwrap().into_denominator(), 3);
    /// assert_eq!(Rational::from_str("0").unwrap().into_denominator(), 1);
    /// ```
    #[inline]
    #[allow(clippy::missing_const_for_fn)]
    pub fn into_denominator(self) -> Natural {
        self.denominator
    }

    /// Extracts the numerator and denominator of a [`Rational`], taking the [`Rational`] by value.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    ///
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_q::Rational;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(
    ///     Rational::from_str("2/3").unwrap().into_numerator_and_denominator().to_debug_string(),
    ///     "(2, 3)"
    /// );
    /// assert_eq!(
    ///     Rational::from_str("0").unwrap().into_numerator_and_denominator().to_debug_string(),
    ///     "(0, 1)"
    /// );
    /// ```
    #[inline]
    #[allow(clippy::missing_const_for_fn)]
    pub fn into_numerator_and_denominator(self) -> (Natural, Natural) {
        (self.numerator, self.denominator)
    }

    /// Returns a reference to the numerator of a [`Rational`].
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_q::Rational;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(*Rational::from_str("2/3").unwrap().numerator_ref(), 2);
    /// assert_eq!(*Rational::from_str("0").unwrap().numerator_ref(), 0);
    /// ```
    #[inline]
    pub const fn numerator_ref(&self) -> &Natural {
        &self.numerator
    }

    /// Returns a reference to the denominator of a [`Rational`].
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_q::Rational;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(*Rational::from_str("2/3").unwrap().denominator_ref(), 3);
    /// assert_eq!(*Rational::from_str("0").unwrap().denominator_ref(), 1);
    /// ```
    #[inline]
    pub const fn denominator_ref(&self) -> &Natural {
        &self.denominator
    }

    /// Returns references to the numeraror and denominator of a [`Rational`].
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    ///
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_q::Rational;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(
    ///     Rational::from_str("2/3").unwrap().numerator_and_denominator_ref().to_debug_string(),
    ///     "(2, 3)"
    /// );
    /// assert_eq!(
    ///     Rational::from_str("0").unwrap().numerator_and_denominator_ref().to_debug_string(),
    ///     "(0, 1)"
    /// );
    /// ```
    #[inline]
    pub const fn numerator_and_denominator_ref(&self) -> (&Natural, &Natural) {
        (&self.numerator, &self.denominator)
    }
}
