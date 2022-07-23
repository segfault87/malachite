use crate::num::arithmetic::traits::{
    OverflowingMulAssign, OverflowingSquare, OverflowingSquareAssign,
};

macro_rules! impl_overflowing_square {
    ($t:ident) => {
        impl OverflowingSquare for $t {
            type Output = $t;

            /// Squares a number.
            ///
            /// Returns a tuple containing the result and a boolean indicating whether an
            /// arithmetic occurred. If an overflow occurred, then the wrapped value is returned.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::overflowing_square#overflowing_square).
            #[inline]
            fn overflowing_square(self) -> ($t, bool) {
                self.overflowing_mul(self)
            }
        }

        impl OverflowingSquareAssign for $t {
            /// Squares a number in place.
            ///
            /// Returns a boolean indicating whether an arithmetic overflow occurred. If an
            /// overflow occurred, then the wrapped value is assigned.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::overflowing_square#overflowing_square_assign).
            #[inline]
            fn overflowing_square_assign(&mut self) -> bool {
                self.overflowing_mul_assign(*self)
            }
        }
    };
}
apply_to_primitive_ints!(impl_overflowing_square);
