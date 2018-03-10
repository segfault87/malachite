use natural::{Large, Natural, Small};

// Returns the length of `limbs`, excluding trailing zeros.
fn limbs_significant_length(limbs: &[u32]) -> usize {
    limbs
        .iter()
        .enumerate()
        .rev()
        .find(|&(_, &limb)| limb != 0)
        .map(|(i, _)| i + 1)
        .unwrap_or(0)
}

impl Natural {
    /// Converts a slice of limbs, or base-2<sup>32</sup> digits, to a `Natural`, in ascending
    /// order, so that less significant limbs have lower indices in the input slice.
    ///
    /// This function borrows `limbs`. If taking ownership of `limbs` is possible,
    /// `from_owned_limbs_asc` is more efficient.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `limbs.len()`
    ///
    /// This method is more efficient than `Natural::from_limbs_desc`.
    ///
    /// # Example
    /// ```
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from_limbs_asc(&[]).to_string(), "0");
    /// assert_eq!(Natural::from_limbs_asc(&[123]).to_string(), "123");
    /// // 10^12 = 232 * 2^32 + 3567587328
    /// assert_eq!(Natural::from_limbs_asc(&[3567587328, 232]).to_string(), "1000000000000");
    /// ```
    pub fn from_limbs_asc(limbs: &[u32]) -> Natural {
        let significant_length = limbs_significant_length(limbs);
        match significant_length {
            0 => Small(0u32),
            1 => Small(limbs[0]),
            _ => Large(limbs[0..significant_length].to_vec()),
        }
    }

    /// Converts a slice of limbs, or base-2<sup>32</sup> digits, to a `Natural`, in descending
    /// order, so that less significant limbs have higher indices in the input slice.
    ///
    /// This function borrows `limbs`. If taking ownership of `limbs` is possible,
    /// `from_owned_limbs_desc` is more efficient.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `limbs.len()`
    ///
    /// This method is less efficient than `Natural::from_limbs_asc`.
    ///
    /// # Example
    /// ```
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from_limbs_desc(&[]).to_string(), "0");
    /// assert_eq!(Natural::from_limbs_desc(&[123]).to_string(), "123");
    /// // 10^12 = 232 * 2^32 + 3567587328
    /// assert_eq!(Natural::from_limbs_desc(&[232, 3567587328]).to_string(), "1000000000000");
    /// ```
    pub fn from_limbs_desc(limbs: &[u32]) -> Natural {
        Natural::from_limbs_asc(&limbs.iter().cloned().rev().collect::<Vec<u32>>())
    }

    /// Converts a `Vec` of limbs, or base-2<sup>32</sup> digits, to a `Natural`, in ascending
    /// order, so that less significant limbs have lower indices in the input `Vec`.
    ///
    /// This function takes ownership of `limbs`. If it's necessary to borrow `limbs` instead, use
    /// `from_limbs_asc`.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// where n = `limbs.len()`
    ///
    /// This method is more efficient than `Natural::from_limbs_desc`.
    ///
    /// # Example
    /// ```
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from_owned_limbs_asc(vec![]).to_string(), "0");
    /// assert_eq!(Natural::from_owned_limbs_asc(vec![123]).to_string(), "123");
    /// // 10^12 = 232 * 2^32 + 3567587328
    /// assert_eq!(Natural::from_owned_limbs_asc(vec![3567587328, 232]).to_string(),
    ///     "1000000000000");
    /// ```
    pub fn from_owned_limbs_asc(mut limbs: Vec<u32>) -> Natural {
        let significant_length = limbs_significant_length(&limbs);
        match significant_length {
            0 => Small(0u32),
            1 => Small(limbs[0]),
            _ => {
                limbs.truncate(significant_length);
                Large(limbs)
            }
        }
    }

    /// Converts a `Vec` of limbs, or base-2<sup>32</sup> digits, to a `Natural`, in descending
    /// order, so that less significant limbs have higher indices in the input `Vec`.
    ///
    /// This function takes ownership of `limbs`. If it's necessary to borrow `limbs` instead, use
    /// `from_limbs_desc`.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// where n = `limbs.len()`
    ///
    /// This method is less efficient than `Natural::from_limbs_asc`.
    ///
    /// # Example
    /// ```
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from_owned_limbs_desc(vec![]).to_string(), "0");
    /// assert_eq!(Natural::from_owned_limbs_desc(vec![123]).to_string(), "123");
    /// // 10^12 = 232 * 2^32 + 3567587328
    /// assert_eq!(Natural::from_owned_limbs_desc(vec![232, 3567587328]).to_string(),
    ///     "1000000000000");
    /// ```
    pub fn from_owned_limbs_desc(mut limbs: Vec<u32>) -> Natural {
        limbs.reverse();
        Natural::from_owned_limbs_asc(limbs)
    }
}