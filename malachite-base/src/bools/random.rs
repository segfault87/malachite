use num::arithmetic::traits::Parity;
use num::random::geometric::SimpleRational;
use num::random::{random_unsigneds_less_than, RandomUnsignedsLessThan};
use rand::Rng;
use rand_chacha::ChaCha20Rng;
use random::Seed;

/// Uniformly generates random `bool`s.
///
/// This `struct` is created by the `random_bools` function. See its documentation for more.
#[derive(Clone, Debug)]
pub struct RandomBools {
    rng: ChaCha20Rng,
    x: u32,
    bits_left: u8,
}

impl Iterator for RandomBools {
    type Item = bool;

    #[inline]
    fn next(&mut self) -> Option<bool> {
        if self.bits_left == 0 {
            self.x = self.rng.gen();
            self.bits_left = 31;
        } else {
            self.x >>= 1;
            self.bits_left -= 1;
        }
        Some(self.x.odd())
    }
}

/// Uniformly generates random `bool`s.
///
/// $P(\text{false}) = P(\text{true}) = \frac{1}{2}$.
///
/// The output length is infinite.
///
/// # Complexity per iteration
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// extern crate itertools;
///
/// use itertools::Itertools;
///
/// use malachite_base::bools::random::random_bools;
/// use malachite_base::random::EXAMPLE_SEED;
///
/// assert_eq!(
///     random_bools(EXAMPLE_SEED).take(10).collect_vec(),
///     &[true, false, false, false, true, true, true, false, true, true]
/// )
/// ```
///
/// # Implementation notes
/// The resulting iterator uses every random bit generated by the PRNG, unlike some implementations
/// which only use one bit out of 32 or 64.
#[inline]
pub fn random_bools(seed: Seed) -> RandomBools {
    RandomBools {
        rng: seed.get_rng(),
        x: 0,
        bits_left: 0,
    }
}

/// Generates random `bool`s, with a fixed probability of generating `true`.
///
/// This `struct` is created by the `weighted_random_bools` function. See its documentation for
/// more.
#[derive(Clone, Debug)]
pub struct WeightedRandomBools {
    numerator: u64,
    xs: RandomUnsignedsLessThan<u64>,
}

impl Iterator for WeightedRandomBools {
    type Item = bool;

    #[inline]
    fn next(&mut self) -> Option<bool> {
        Some(self.xs.next().unwrap() < self.numerator)
    }
}

/// Generates random `bool`s, with a fixed probability of generating `true`.
///
/// The probability of generating `true` is $p$ = `p_numerator` / `p_denominator`.
///
/// $P(\text{true}) = p$
///
/// $P(\text{false}) = 1-p$
///
/// The output length is infinite.
///
/// # Panics
/// Panics if `p_denominator` is 0 or `p_numerator` > `p_denominator`.
///
/// # Expected complexity per iteration
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// extern crate itertools;
///
/// use itertools::Itertools;
///
/// use malachite_base::bools::random::weighted_random_bools;
/// use malachite_base::random::EXAMPLE_SEED;
///
/// assert_eq!(
///     weighted_random_bools(EXAMPLE_SEED, 3, 4).take(10).collect_vec(),
///     &[true, true, false, true, false, false, true, false, true, true]
/// )
/// ```
pub fn weighted_random_bools(
    seed: Seed,
    p_numerator: u64,
    p_denominator: u64,
) -> WeightedRandomBools {
    assert!(p_numerator <= p_denominator);
    let p = SimpleRational::new(p_numerator, p_denominator);
    WeightedRandomBools {
        numerator: p.n,
        xs: random_unsigneds_less_than(seed, p.d),
    }
}
