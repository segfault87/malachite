use malachite_base::num::arithmetic::traits::ShlRound;
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::{
    CheckedFrom, ConvertibleFrom, IntegerMantissaAndExponent, IsInteger, RoundingFrom,
};
use malachite_base::rounding_modes::RoundingMode;
use natural::Natural;

macro_rules! float_impls {
    ($f: ident) => {
        impl RoundingFrom<$f> for Natural {
            /// Converts a floating-point value to a [`Natural`], using the specified rounding
            /// mode.
            ///
            /// The floating-point value cannot be NaN or infinite, and it cannot round to a
            /// negative integer.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `value.sci_exponent()`.
            ///
            /// # Panics
            /// Panics if `value` is NaN or infinite, if it would round to a negative integer, or if
            /// the rounding mode is `Exact` and `value` is not an integer.
            ///
            /// # Examples
            /// See [here](super::from_primitive_float#rounding_from).
            fn rounding_from(value: $f, rm: RoundingMode) -> Self {
                if value.is_nan() || value == $f::POSITIVE_INFINITY {
                    panic!("Cannot convert {} to Natural", value);
                } else if value == 0.0 {
                    Natural::ZERO
                } else if value < 0.0 {
                    if rm == RoundingMode::Down
                        || rm == RoundingMode::Ceiling
                        || rm == RoundingMode::Nearest
                    {
                        Natural::ZERO
                    } else {
                        panic!("Result is negative and cannot be converted to a Natural");
                    }
                } else {
                    let (mantissa, exponent) = value.integer_mantissa_and_exponent();
                    Natural::from(mantissa).shl_round(exponent, rm)
                }
            }
        }

        impl From<$f> for Natural {
            /// Converts a floating-point value to the nearest [`Natural`].
            ///
            /// Floating-point values exactly between two [`Natural`]s are rounded to the even one.
            /// The floating point value cannot be NaN or infinite, and it cannot round to a
            /// negative integer (so it must be greater than or equal to -0.5).
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `value.sci_exponent()`.
            ///
            /// # Panics
            /// Panics if `value` is NaN or infinite, or if it would round to a negative integer.
            ///
            /// # Examples
            /// See [here](super::from_primitive_float#from).
            fn from(value: $f) -> Natural {
                Natural::rounding_from(value, RoundingMode::Nearest)
            }
        }

        impl CheckedFrom<$f> for Natural {
            /// Converts a floating-point value to a [`Natural`] type.
            ///
            /// If the input isn't exactly equal to some [`Natural`], `None` is returned.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `value.sci_exponent()`.
            ///
            /// # Examples
            /// See [here](super::from_primitive_float#checked_from).
            fn checked_from(value: $f) -> Option<Natural> {
                if value.is_nan() || value.is_infinite() || value < 0.0 {
                    None
                } else if value == 0.0 {
                    Some(Natural::ZERO)
                } else {
                    let (mantissa, exponent) = value.integer_mantissa_and_exponent();
                    if exponent >= 0 {
                        Some(Natural::from(mantissa) << exponent)
                    } else {
                        None
                    }
                }
            }
        }

        impl ConvertibleFrom<$f> for Natural {
            /// Determines whether a floating-point value can be exactly converted to a
            /// [`Natural`].
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::from_primitive_float#convertible_from).
            #[inline]
            fn convertible_from(value: $f) -> bool {
                value >= 0.0 && value.is_integer()
            }
        }
    };
}
apply_to_primitive_floats!(float_impls);