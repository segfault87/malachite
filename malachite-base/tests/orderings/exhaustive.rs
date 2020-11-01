use std::cmp::Ordering;

use malachite_base::orderings::exhaustive::{exhaustive_orderings, orderings_increasing};

#[test]
fn test_orderings_increasing() {
    assert_eq!(
        orderings_increasing().collect::<Vec<_>>(),
        &[Ordering::Less, Ordering::Equal, Ordering::Greater]
    );
}

#[test]
fn test_exhaustive_orderings() {
    assert_eq!(
        exhaustive_orderings().collect::<Vec<_>>(),
        &[Ordering::Equal, Ordering::Less, Ordering::Greater]
    );
}