use crate::common::{rational_to_bigrational, rational_to_rug_rational};
use malachite_base_test_util::generators::common::It;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz_test_util::common::{integer_to_rug_integer, natural_to_rug_integer};
use malachite_q::Rational;
use num::BigRational;

pub fn rational_nrm(xs: It<Rational>) -> It<(BigRational, rug::Rational, Rational)> {
    Box::new(xs.map(|x| (rational_to_bigrational(&x), rational_to_rug_rational(&x), x)))
}

pub fn rational_pair_rm(
    ps: It<(Rational, Rational)>,
) -> It<((rug::Rational, rug::Rational), (Rational, Rational))> {
    Box::new(ps.map(|(x, y)| {
        (
            (rational_to_rug_rational(&x), rational_to_rug_rational(&y)),
            (x, y),
        )
    }))
}

#[allow(clippy::type_complexity)]
pub fn rational_pair_nrm(
    ps: It<(Rational, Rational)>,
) -> It<(
    (BigRational, BigRational),
    (rug::Rational, rug::Rational),
    (Rational, Rational),
)> {
    Box::new(ps.map(|(x, y)| {
        (
            (rational_to_bigrational(&x), rational_to_bigrational(&y)),
            (rational_to_rug_rational(&x), rational_to_rug_rational(&y)),
            (x, y),
        )
    }))
}

pub fn rational_integer_pair_rm(
    ps: It<(Rational, Integer)>,
) -> It<((rug::Rational, rug::Integer), (Rational, Integer))> {
    Box::new(ps.map(|(x, y)| {
        (
            (rational_to_rug_rational(&x), integer_to_rug_integer(&y)),
            (x, y),
        )
    }))
}

pub fn rational_natural_pair_rm(
    ps: It<(Rational, Natural)>,
) -> It<((rug::Rational, rug::Integer), (Rational, Natural))> {
    Box::new(ps.map(|(x, y)| {
        (
            (rational_to_rug_rational(&x), natural_to_rug_integer(&y)),
            (x, y),
        )
    }))
}

pub fn rational_pair_1_rm<T: 'static + Clone>(
    ps: It<(Rational, T)>,
) -> It<((rug::Rational, T), (Rational, T))> {
    Box::new(ps.map(|(x, y)| ((rational_to_rug_rational(&x), y.clone()), (x, y))))
}
