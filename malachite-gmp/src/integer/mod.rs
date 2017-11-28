use gmp_mpfr_sys::gmp::{self, mpz_t};
use integer::Integer::*;
use malachite_base::traits::{NegativeOne, One, Two, Zero};
use std::mem;

/// An integer backed by [GMP](https://gmplib.org/).
///
/// This code uses Trevor Spiteri's
/// [`gmp_mpfr_sys`](https://tspiteri.gitlab.io/gmp-mpfr/gmp_mpfr_sys/index.html) low-level
/// bindings.
///
/// Any `Integer` small enough to fit into an `i32` is represented inline. Only integers outside
/// this range incur the costs of FFI and heap-allocation.
pub enum Integer {
    /// A small `Integer`.
    Small(i32),
    /// A large `Integer`.
    Large(mpz_t),
}

impl Integer {
    pub fn new_mpz_t() -> mpz_t {
        let mut x: mpz_t = unsafe { mem::uninitialized() };
        unsafe {
            gmp::mpz_init(&mut x);
        }
        x
    }

    fn promote(&self) -> Integer {
        match self {
            &Small(x) => unsafe {
                let mut promoted: mpz_t = mem::uninitialized();
                gmp::mpz_init_set_si(&mut promoted, x.into());
                Large(promoted)
            },
            x => x.clone(),
        }
    }

    fn promote_in_place(&mut self) -> &mut mpz_t {
        if let Small(x) = *self {
            unsafe {
                let mut promoted: mpz_t = mem::uninitialized();
                gmp::mpz_init_set_si(&mut promoted, x.into());
                *self = Large(promoted);
            }
        }
        if let Large(ref mut x) = *self {
            return x;
        }
        unreachable!();
    }

    fn demote_if_small(&mut self) {
        if let Large(x) = *self {
            if unsafe {
                gmp::mpz_cmp_si(&x, i32::min_value().into()) >= 0 &&
                    gmp::mpz_cmp_si(&x, i32::max_value().into()) <= 0
            }
            {
                let s = (unsafe { gmp::mpz_get_si(&x) }) as i32;
                *self = Small(s);
            }
        }
    }

    fn assign_mpz_t(&mut self, x: mpz_t) {
        *self = Large(x);
        self.demote_if_small();
    }

    /// Returns true iff `self` is valid. To be valid, `self` can only be Large when it is less than
    /// -2^(31) or at least 2^(31). All Integers must be valid.
    pub fn is_valid(&self) -> bool {
        //TODO better range check
        match *self {
            Small(_) => true,
            Large(ref x) => {
                (unsafe { gmp::mpz_cmp_si(x, i32::min_value().into()) }) < 0 ||
                    (unsafe { gmp::mpz_cmp_si(x, i32::max_value().into()) }) > 0
            }
        }
    }
}

/// If `self` is large, clears the GMP-allocated memory.
impl Drop for Integer {
    fn drop(&mut self) {
        if let Large(ref mut x) = *self {
            unsafe {
                gmp::mpz_clear(x);
            }
        }
    }
}

/// The constant 0.
impl Zero for Integer {
    const ZERO: Integer = Small(0);
}

/// The constant 1.
impl One for Integer {
    const ONE: Integer = Small(1);
}

/// The constant 2.
impl Two for Integer {
    const TWO: Integer = Small(2);
}

/// The constant -1.
impl NegativeOne for Integer {
    const NEGATIVE_ONE: Integer = Small(-1);
}

macro_rules! mutate_with_possible_promotion {
    ($n: ident, $small: ident, $large: ident, $process_small: expr, $process_large: expr) => {
        if let Small(ref mut $small) = *$n {
            if let Some(x) = $process_small {
                *$small = x;
                return;
            }
        }
        if let Small(x) = *$n {
            unsafe {
                let mut promoted: mpz_t = mem::uninitialized();
                gmp::mpz_init_set_si(&mut promoted, x.into());
                *$n = Large(promoted);
            }
        }
        if let Large(ref mut $large) = *$n {
            $process_large
        }
    };
}

pub mod arithmetic;
pub mod comparison {
    pub mod eq;
    pub mod hash;
    pub mod ord;
    pub mod ord_abs;
    pub mod partial_eq_i32;
    pub mod partial_eq_natural;
    pub mod partial_eq_u32;
    pub mod partial_ord_abs_i32;
    pub mod partial_ord_abs_natural;
    pub mod partial_ord_abs_u32;
    pub mod partial_ord_i32;
    pub mod partial_ord_natural;
    pub mod partial_ord_u32;
    pub mod sign;
}
pub mod conversion;
pub mod logic;
pub mod random;
