use common::{GenerationMode, NoSpecialGenerationMode};
use inputs::common::{permute_1_3_2, permute_2_1, reshape_2_1_to_3};
use malachite_base::chars::NUMBER_OF_CHARS;
use malachite_base::limbs::limbs_test_zero;
use malachite_base::num::{PrimitiveInteger, PrimitiveSigned, PrimitiveUnsigned};
use malachite_base::round::RoundingMode;
use rust_wheels::iterators::bools::exhaustive_bools;
use rust_wheels::iterators::chars::exhaustive_chars;
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::general::{random, range_increasing};
use rust_wheels::iterators::integers_geometric::u32s_geometric;
use rust_wheels::iterators::orderings::{exhaustive_orderings, random_orderings};
use rust_wheels::iterators::primitive_ints::{exhaustive_negative_signed, exhaustive_positive,
                                             exhaustive_signed, exhaustive_unsigned,
                                             random_negative_signed, random_positive_signed,
                                             random_positive_unsigned, random_range,
                                             special_random_negative_signed,
                                             special_random_positive_signed,
                                             special_random_positive_unsigned,
                                             special_random_signed, special_random_unsigned};
use rust_wheels::iterators::rounding_modes::{exhaustive_rounding_modes, random_rounding_modes};
use rust_wheels::iterators::tuples::{exhaustive_pairs, exhaustive_pairs_from_single, lex_pairs,
                                     lex_triples, log_pairs, random_pairs,
                                     random_pairs_from_single, random_triples,
                                     random_triples_from_single, sqrt_pairs};
use rust_wheels::iterators::vecs::{exhaustive_vecs, random_vecs, special_random_unsigned_vecs};
use std::char;
use std::cmp::Ordering;

type It<T> = Box<Iterator<Item = T>>;

pub fn unsigneds<T: 'static + PrimitiveUnsigned>(gm: GenerationMode) -> It<T> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_unsigned()),
        GenerationMode::Random(_) => Box::new(random(&EXAMPLE_SEED)),
        GenerationMode::SpecialRandom(_) => Box::new(special_random_unsigned(&EXAMPLE_SEED)),
    }
}

pub fn signeds<T: 'static + PrimitiveSigned>(gm: GenerationMode) -> It<T> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_signed()),
        GenerationMode::Random(_) => Box::new(random(&EXAMPLE_SEED)),
        GenerationMode::SpecialRandom(_) => Box::new(special_random_signed(&EXAMPLE_SEED)),
    }
}

pub fn positive_unsigneds<T: 'static + PrimitiveUnsigned>(gm: GenerationMode) -> It<T> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_positive()),
        GenerationMode::Random(_) => Box::new(random_positive_unsigned(&EXAMPLE_SEED)),
        GenerationMode::SpecialRandom(_) => {
            Box::new(special_random_positive_unsigned(&EXAMPLE_SEED))
        }
    }
}

pub fn unsigneds_no_max<T: 'static + PrimitiveUnsigned>(gm: GenerationMode) -> It<T> {
    Box::new(unsigneds(gm).filter(|&u| u != T::MAX))
}

pub fn signeds_no_max<T: 'static + PrimitiveSigned>(gm: GenerationMode) -> It<T> {
    Box::new(signeds(gm).filter(|&i| i != T::MAX))
}

pub fn signeds_no_min<T: 'static + PrimitiveSigned>(gm: GenerationMode) -> It<T> {
    Box::new(signeds(gm).filter(|&i| i != T::MIN))
}

pub fn pairs_of_unsigneds<T: 'static + PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (T, T)>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_pairs_from_single(exhaustive_unsigned())),
        GenerationMode::Random(_) => Box::new(random_pairs_from_single(random(&EXAMPLE_SEED))),
        GenerationMode::SpecialRandom(_) => Box::new(random_pairs_from_single(
            special_random_unsigned(&EXAMPLE_SEED),
        )),
    }
}

// All `u32`s smaller than `NUMBER_OF_CHARS`.
pub fn u32s_range_1(gm: NoSpecialGenerationMode) -> It<u32> {
    match gm {
        NoSpecialGenerationMode::Exhaustive => Box::new(range_increasing(0, NUMBER_OF_CHARS - 1)),
        NoSpecialGenerationMode::Random(_) => {
            Box::new(random_range(&EXAMPLE_SEED, 0, NUMBER_OF_CHARS - 1))
        }
    }
}

// All pairs of `u32`s smaller than `NUMBER_OF_CHARS`.
pub fn pairs_of_u32s_range_1(gm: NoSpecialGenerationMode) -> Box<Iterator<Item = (u32, u32)>> {
    match gm {
        NoSpecialGenerationMode::Exhaustive => Box::new(exhaustive_pairs_from_single(
            range_increasing(0, NUMBER_OF_CHARS - 1),
        )),
        NoSpecialGenerationMode::Random(_) => Box::new(random_pairs_from_single(random_range(
            &EXAMPLE_SEED,
            0,
            NUMBER_OF_CHARS - 1,
        ))),
    }
}

pub fn small_u32s(gm: NoSpecialGenerationMode) -> Box<Iterator<Item = u32>> {
    match gm {
        NoSpecialGenerationMode::Exhaustive => Box::new(exhaustive_unsigned()),
        NoSpecialGenerationMode::Random(scale) => Box::new(u32s_geometric(&EXAMPLE_SEED, scale)),
    }
}

pub fn small_u64s(gm: NoSpecialGenerationMode) -> Box<Iterator<Item = u64>> {
    match gm {
        NoSpecialGenerationMode::Exhaustive => Box::new(exhaustive_unsigned()),
        NoSpecialGenerationMode::Random(scale) => {
            Box::new(u32s_geometric(&EXAMPLE_SEED, scale).map(|i| i.into()))
        }
    }
}

fn sqrt_pairs_of_unsigneds<T: 'static + PrimitiveUnsigned, U: 'static + PrimitiveUnsigned>(
) -> Box<Iterator<Item = (T, U)>> {
    Box::new(sqrt_pairs(exhaustive_unsigned(), exhaustive_unsigned()))
}

fn random_pairs_of_primitive_and_geometric_u32<T: 'static + PrimitiveInteger>(
    scale: u32,
) -> Box<Iterator<Item = (T, u32)>> {
    Box::new(random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random(seed)),
        &(|seed| u32s_geometric(seed, scale)),
    ))
}

fn random_pairs_of_primitive_and_geometric_u64<T: 'static + PrimitiveInteger>(
    scale: u32,
) -> It<(T, u64)> {
    Box::new(random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random(seed)),
        &(|seed| u32s_geometric(seed, scale).map(|i| i.into())),
    ))
}

pub fn pairs_of_unsigned_and_small_u32<T: 'static + PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (T, u32)>> {
    match gm {
        GenerationMode::Exhaustive => sqrt_pairs_of_unsigneds(),
        GenerationMode::Random(scale) => random_pairs_of_primitive_and_geometric_u32(scale),
        GenerationMode::SpecialRandom(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned(seed)),
            &(|seed| u32s_geometric(seed, scale)),
        )),
    }
}

pub fn pairs_of_unsigned_and_small_u64<T: 'static + PrimitiveUnsigned>(
    gm: GenerationMode,
) -> It<(T, u64)> {
    match gm {
        GenerationMode::Exhaustive => sqrt_pairs_of_unsigneds(),
        GenerationMode::Random(scale) => random_pairs_of_primitive_and_geometric_u64(scale),
        GenerationMode::SpecialRandom(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned(seed)),
            &(|seed| u32s_geometric(seed, scale).map(|i| i.into())),
        )),
    }
}

pub fn pairs_of_small_usize_and_unsigned<T: 'static + PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (usize, T)>> {
    match gm {
        GenerationMode::Exhaustive => permute_2_1(Box::new(log_pairs(
            exhaustive_unsigned(),
            exhaustive_unsigned::<u32>().map(|u| u as usize),
        ))),
        GenerationMode::Random(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| u32s_geometric(seed, scale).map(|u| u as usize)),
            &(|seed| random(seed)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| u32s_geometric(seed, scale).map(|u| u as usize)),
            &(|seed| special_random_unsigned(seed)),
        )),
    }
}

fn log_pairs_of_positive_primitive_and_unsigned<
    T: 'static + PrimitiveInteger,
    U: 'static + PrimitiveUnsigned,
>() -> Box<Iterator<Item = (T, U)>> {
    Box::new(log_pairs(exhaustive_positive(), exhaustive_unsigned()))
}

pub fn pairs_of_positive_unsigned_and_small_u32<T: 'static + PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (T, u32)>> {
    match gm {
        GenerationMode::Exhaustive => log_pairs_of_positive_primitive_and_unsigned(),
        GenerationMode::Random(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_positive_unsigned(seed)),
            &(|seed| u32s_geometric(seed, scale)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| special_random_positive_unsigned(seed)),
            &(|seed| u32s_geometric(seed, scale)),
        )),
    }
}

pub fn pairs_of_positive_signed_and_small_u32<T: 'static + PrimitiveSigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (T, u32)>> {
    match gm {
        GenerationMode::Exhaustive => log_pairs_of_positive_primitive_and_unsigned(),
        GenerationMode::Random(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_positive_signed(seed)),
            &(|seed| u32s_geometric(seed, scale)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| special_random_positive_signed(seed)),
            &(|seed| u32s_geometric(seed, scale)),
        )),
    }
}

fn sqrt_pairs_of_signed_and_unsigned<
    T: 'static + PrimitiveSigned,
    U: 'static + PrimitiveUnsigned,
>() -> Box<Iterator<Item = (T, U)>> {
    Box::new(sqrt_pairs(exhaustive_signed(), exhaustive_unsigned()))
}

pub fn pairs_of_signed_and_small_u64<T: 'static + PrimitiveSigned>(
    gm: GenerationMode,
) -> It<(T, u64)> {
    match gm {
        GenerationMode::Exhaustive => sqrt_pairs_of_signed_and_unsigned(),
        GenerationMode::Random(scale) => random_pairs_of_primitive_and_geometric_u64(scale),
        GenerationMode::SpecialRandom(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| special_random_signed(seed)),
            &(|seed| u32s_geometric(seed, scale).map(|i| i.into())),
        )),
    }
}

type ItU<T> = It<(T, u64)>;
fn exhaustive_pairs_of_unsigned_and_u64_width_range<T: 'static + PrimitiveUnsigned>() -> ItU<T> {
    Box::new(lex_pairs(
        exhaustive_unsigned(),
        range_increasing(0, u64::from(T::WIDTH) - 1),
    ))
}

fn random_pairs_of_primitive_and_u64_width_range<T: 'static + PrimitiveInteger>() -> It<(T, u64)> {
    Box::new(random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random(seed)),
        &(|seed| random_range(seed, 0, u64::from(T::WIDTH) - 1)),
    ))
}

// All pairs of unsigned `T` and `u64`, where the `u64` is smaller that `T::WIDTH`.
pub fn pairs_of_unsigned_and_u64_width_range<T: 'static + PrimitiveUnsigned>(
    gm: GenerationMode,
) -> It<(T, u64)> {
    match gm {
        GenerationMode::Exhaustive => exhaustive_pairs_of_unsigned_and_u64_width_range(),
        GenerationMode::Random(_) => random_pairs_of_primitive_and_u64_width_range(),
        GenerationMode::SpecialRandom(_) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned(seed)),
            &(|seed| random_range(seed, 0, u64::from(T::WIDTH) - 1)),
        )),
    }
}

// All pairs of signed `T` and `u64`, where the `u64` is smaller that `T::WIDTH`.
pub fn pairs_of_signed_and_u64_width_range<T: 'static + PrimitiveSigned>(
    gm: GenerationMode,
) -> It<(T, u64)> {
    match gm {
        GenerationMode::Exhaustive => Box::new(lex_pairs(
            exhaustive_signed(),
            range_increasing(0, u64::from(T::WIDTH) - 1),
        )),
        GenerationMode::Random(_) => random_pairs_of_primitive_and_u64_width_range(),
        GenerationMode::SpecialRandom(_) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| special_random_signed(seed)),
            &(|seed| random_range(seed, 0, u64::from(T::WIDTH) - 1)),
        )),
    }
}

// All pairs of signed `T` and `u64`, where the signed `T` i s negative or the `u64` is smaller than
// `T::WIDTH`.
pub fn pairs_of_signed_and_u64_width_range_var_1<T: 'static + PrimitiveSigned>(
    gm: GenerationMode,
) -> It<(T, u64)> {
    Box::new(
        pairs_of_signed_and_small_u64(gm)
            .filter(|&(n, index)| n < T::ZERO || index < u64::from(T::WIDTH)),
    )
}

// All pairs of signed `T` and `u64`, where the signed `T` i s non-negative or the `u64` is smaller
// than `T::WIDTH`.
pub fn pairs_of_signed_and_u64_width_range_var_2<T: 'static + PrimitiveSigned>(
    gm: GenerationMode,
) -> It<(T, u64)> {
    Box::new(
        pairs_of_signed_and_small_u64(gm)
            .filter(|&(n, index)| n >= T::ZERO || index < u64::from(T::WIDTH)),
    )
}

// All triples of unsigned `T`, `u64`, and `bool`, where the `bool` is false or the `u64` is smaller
// than `T::WIDTH`.
pub fn triples_of_unsigned_u64_width_range_and_bool_var_1<T: 'static + PrimitiveUnsigned>(
    gm: GenerationMode,
) -> It<(T, u64, bool)> {
    let unfiltered: It<(T, u64, bool)> = match gm {
        GenerationMode::Exhaustive => reshape_2_1_to_3(Box::new(lex_pairs(
            sqrt_pairs_of_unsigneds(),
            exhaustive_bools(),
        ))),
        GenerationMode::Random(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| random(seed)),
            &(|seed| u32s_geometric(seed, scale).map(|i| i.into())),
            &(|seed| random(seed)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned(seed)),
            &(|seed| u32s_geometric(seed, scale).map(|i| i.into())),
            &(|seed| random(seed)),
        )),
    };
    Box::new(unfiltered.filter(|&(_, index, bit)| !bit || index < u64::from(T::WIDTH)))
}

// All triples of signed `T`, `u64`, and `bool`, where the `u64` is smaller than `T::WIDTH` or the
// `bool` is equal to whether the `T` is negative.
pub fn triples_of_signed_u64_width_range_and_bool_var_1<T: 'static + PrimitiveSigned>(
    gm: GenerationMode,
) -> It<(T, u64, bool)> {
    let unfiltered: It<(T, u64, bool)> = match gm {
        GenerationMode::Exhaustive => reshape_2_1_to_3(Box::new(lex_pairs(
            sqrt_pairs_of_signed_and_unsigned(),
            exhaustive_bools(),
        ))),
        GenerationMode::Random(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| random(seed)),
            &(|seed| u32s_geometric(seed, scale).map(|i| i.into())),
            &(|seed| random(seed)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| special_random_signed(seed)),
            &(|seed| u32s_geometric(seed, scale).map(|i| i.into())),
            &(|seed| random(seed)),
        )),
    };
    Box::new(
        unfiltered.filter(|&(n, index, bit)| index < u64::from(T::WIDTH) || bit == (n < T::ZERO)),
    )
}

pub fn pairs_of_negative_signed_not_min_and_small_u32s<T: 'static + PrimitiveSigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (T, u32)>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(log_pairs(
            exhaustive_negative_signed().filter(|&i| i != T::MIN),
            exhaustive_unsigned(),
        )),
        GenerationMode::Random(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_negative_signed(seed).filter(|&i| i != T::MIN)),
            &(|seed| u32s_geometric(seed, scale)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| special_random_negative_signed(seed).filter(|&i| i != T::MIN)),
            &(|seed| u32s_geometric(seed, scale)),
        )),
    }
}

pub fn chars(gm: NoSpecialGenerationMode) -> Box<Iterator<Item = char>> {
    match gm {
        NoSpecialGenerationMode::Exhaustive => Box::new(exhaustive_chars()),
        NoSpecialGenerationMode::Random(_) => Box::new(random(&EXAMPLE_SEED)),
    }
}

pub fn chars_not_min(gm: NoSpecialGenerationMode) -> Box<Iterator<Item = char>> {
    Box::new(chars(gm).filter(|&c| c != '\u{0}'))
}

pub fn chars_not_max(gm: NoSpecialGenerationMode) -> Box<Iterator<Item = char>> {
    Box::new(chars(gm).filter(|&c| c != char::MAX))
}

pub fn pairs_of_chars(gm: NoSpecialGenerationMode) -> Box<Iterator<Item = (char, char)>> {
    match gm {
        NoSpecialGenerationMode::Exhaustive => {
            Box::new(exhaustive_pairs_from_single(exhaustive_chars()))
        }
        NoSpecialGenerationMode::Random(_) => {
            Box::new(random_pairs_from_single(random(&EXAMPLE_SEED)))
        }
    }
}

pub fn rounding_modes(gm: NoSpecialGenerationMode) -> Box<Iterator<Item = RoundingMode>> {
    match gm {
        NoSpecialGenerationMode::Exhaustive => Box::new(exhaustive_rounding_modes()),
        NoSpecialGenerationMode::Random(_) => Box::new(random_rounding_modes(&EXAMPLE_SEED)),
    }
}

pub fn pairs_of_rounding_modes(
    gm: NoSpecialGenerationMode,
) -> Box<Iterator<Item = (RoundingMode, RoundingMode)>> {
    match gm {
        NoSpecialGenerationMode::Exhaustive => Box::new(lex_pairs(
            exhaustive_rounding_modes(),
            exhaustive_rounding_modes(),
        )),
        NoSpecialGenerationMode::Random(_) => Box::new(random_pairs_from_single(
            random_rounding_modes(&EXAMPLE_SEED),
        )),
    }
}

pub fn triples_of_rounding_modes(
    gm: NoSpecialGenerationMode,
) -> Box<Iterator<Item = (RoundingMode, RoundingMode, RoundingMode)>> {
    match gm {
        NoSpecialGenerationMode::Exhaustive => Box::new(lex_triples(
            exhaustive_rounding_modes(),
            exhaustive_rounding_modes(),
            exhaustive_rounding_modes(),
        )),
        NoSpecialGenerationMode::Random(_) => Box::new(random_triples_from_single(
            random_rounding_modes(&EXAMPLE_SEED),
        )),
    }
}

fn random_pairs_of_primitive_and_rounding_mode<T: 'static + PrimitiveInteger>(
) -> Box<Iterator<Item = (T, RoundingMode)>> {
    Box::new(random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random(seed)),
        &(|seed| random_rounding_modes(seed)),
    ))
}

pub fn pairs_of_unsigned_and_rounding_mode<T: 'static + PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (T, RoundingMode)>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(lex_pairs(
            exhaustive_unsigned(),
            exhaustive_rounding_modes(),
        )),
        GenerationMode::Random(_) => random_pairs_of_primitive_and_rounding_mode(),
        GenerationMode::SpecialRandom(_) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned(seed)),
            &(|seed| random_rounding_modes(seed)),
        )),
    }
}

pub fn pairs_of_signed_and_rounding_mode<T: 'static + PrimitiveSigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (T, RoundingMode)>> {
    match gm {
        GenerationMode::Exhaustive => {
            Box::new(lex_pairs(exhaustive_signed(), exhaustive_rounding_modes()))
        }
        GenerationMode::Random(_) => random_pairs_of_primitive_and_rounding_mode(),
        GenerationMode::SpecialRandom(_) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| special_random_signed(seed)),
            &(|seed| random_rounding_modes(seed)),
        )),
    }
}

pub fn vecs_of_unsigned<T: 'static + PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = Vec<T>>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_vecs(exhaustive_unsigned())),
        GenerationMode::Random(scale) => {
            Box::new(random_vecs(&EXAMPLE_SEED, scale, &(|seed| random(seed))))
        }
        GenerationMode::SpecialRandom(scale) => {
            Box::new(special_random_unsigned_vecs(&EXAMPLE_SEED, scale))
        }
    }
}

// All `Vec<T>`, where `T` is unsigned, the `Vec` is nonempty, and its last `T` is nonzero.
pub fn vecs_of_unsigned_var_1<T: 'static + PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = Vec<T>>> {
    Box::new(
        vecs_of_unsigned(gm).filter(|limbs| !limbs.is_empty() && *limbs.last().unwrap() != T::ZERO),
    )
}

fn pairs_of_ordering_and_vec_of_unsigned<T: 'static + PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Ordering, Vec<T>)>> {
    match gm {
        GenerationMode::Exhaustive => permute_2_1(Box::new(lex_pairs(
            exhaustive_vecs(exhaustive_unsigned()),
            exhaustive_orderings(),
        ))),
        GenerationMode::Random(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_orderings(seed)),
            &(|seed| random_vecs(seed, scale, &(|seed_2| random(seed_2)))),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_orderings(seed)),
            &(|seed| special_random_unsigned_vecs(seed, scale)),
        )),
    }
}

// All pairs of `Ordering` and `Vec<T>` where `T` is unsigned, such that the `Ordering` is
// `Ordering::Equal` iff all `T`s in the `Vec` are zero.
pub fn pairs_of_ordering_and_vec_of_unsigned_var_1(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Ordering, Vec<u32>)>> {
    Box::new(
        pairs_of_ordering_and_vec_of_unsigned(gm)
            .filter(|&(sign, ref limbs)| limbs_test_zero(limbs) == (sign == Ordering::Equal)),
    )
}

fn exhaustive_pairs_of_unsigned_vec_and_unsigned<T: 'static + PrimitiveUnsigned>(
) -> Box<Iterator<Item = (Vec<T>, T)>> {
    Box::new(exhaustive_pairs(
        exhaustive_vecs(exhaustive_unsigned()),
        exhaustive_unsigned(),
    ))
}

pub fn triples_of_unsigned_vec_small_usize_and_unsigned<T: 'static + PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Vec<T>, usize, T)>> {
    match gm {
        GenerationMode::Exhaustive => permute_1_3_2(reshape_2_1_to_3(Box::new(log_pairs(
            exhaustive_pairs_of_unsigned_vec_and_unsigned(),
            exhaustive_unsigned::<u32>().map(|u| u as usize),
        )))),
        GenerationMode::Random(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| random_vecs(seed, scale, &(|seed_2| random(seed_2)))),
            &(|seed| u32s_geometric(seed, scale).map(|u| u as usize)),
            &(|seed| random(seed)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned_vecs(seed, scale)),
            &(|seed| u32s_geometric(seed, scale).map(|u| u as usize)),
            &(|seed| special_random_unsigned(seed)),
        )),
    }
}

fn pairs_of_unsigned_vec_and_small_usize<T: 'static + PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Vec<T>, usize)>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(log_pairs(
            exhaustive_vecs(exhaustive_unsigned()),
            exhaustive_unsigned::<u32>().map(|u| u as usize),
        )),
        GenerationMode::Random(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_vecs(seed, scale, &(|seed_2| random(seed_2)))),
            &(|seed| u32s_geometric(seed, scale).map(|u| u as usize)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned_vecs(seed, scale)),
            &(|seed| u32s_geometric(seed, scale).map(|u| u as usize)),
        )),
    }
}

// All pairs of `Vec<T>`, where `T` is unsigned, and `usize, where the `usize` is no larger than the
// length of the `Vec`.
pub fn pairs_of_unsigned_vec_and_small_usize_var_1<T: 'static + PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Vec<T>, usize)>> {
    Box::new(pairs_of_unsigned_vec_and_small_usize(gm).filter(|&(ref xs, u)| u <= xs.len()))
}

pub fn pairs_of_unsigned_vec_and_unsigned<T: 'static + PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Vec<T>, T)>> {
    match gm {
        GenerationMode::Exhaustive => exhaustive_pairs_of_unsigned_vec_and_unsigned(),
        GenerationMode::Random(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_vecs(seed, scale, &(|seed_2| random(seed_2)))),
            &(|seed| random(seed)),
        )),
        GenerationMode::SpecialRandom(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| special_random_unsigned_vecs(seed, scale)),
            &(|seed| special_random_unsigned(seed)),
        )),
    }
}