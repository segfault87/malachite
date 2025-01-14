use crate::Rational;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::IntegerMantissaAndExponent;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RationalFromPrimitiveFloatError;

macro_rules! float_impls {
    ($f: ident) => {
        impl TryFrom<$f> for Rational {
            type Error = RationalFromPrimitiveFloatError;

            /// Converts a primitive float to the equivalent [`Rational`]. If the floating point
            /// value is `NaN` or infinite, and error is returned.
            ///
            /// This conversion is literal. For example, `Rational::try_from(0.1f32)` evaluates to
            /// Some($13421773/134217728$). If you want $1/10$ instead, use
            /// [`try_from_float_simplest`](Rational::try_from_float_simplest); that function
            /// returns the simplest [`Rational`] that rounds to the specified float.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is
            /// `value.sci_exponent().abs()`.
            ///
            /// # Examples
            /// See [here](super::from_primitive_float#try_from).
            fn try_from(value: $f) -> Result<Rational, Self::Error> {
                if !value.is_finite() {
                    Err(RationalFromPrimitiveFloatError)
                } else if value == 0.0 {
                    Ok(Rational::ZERO)
                } else {
                    let (mantissa, exponent) = value.integer_mantissa_and_exponent();
                    let x = Rational::from(mantissa) << exponent;
                    Ok(if value > 0.0 { x } else { -x })
                }
            }
        }
    };
}
apply_to_primitive_floats!(float_impls);
