use std::ops::BitXorAssign;

use itertools::Itertools;
use num_traits::{PrimInt, Unsigned};

use super::{ConvertibleToMask, FindFirstUniqueSet};

/// Implement the [`FindFirstUniqueSet`] trait for any iterator of elements
/// that can be converted to a mask, and are [`PartialEq`] and [`Clone`].
///
/// [`Clone`] is required because we need to [`Itertools::tee`] the iterator
/// to operate on both the left and right sides of the window simultaneously.
impl<T, I> FindFirstUniqueSet<T> for I
where
    I: Iterator<Item = T>,
{
    /// Find the index first unique set of elements in a collection.
    ///
    /// The index is the starting index of the first unique set of elements.
    fn find_first_unique_set_index<const S: usize, M>(self) -> Option<usize>
    where
        T: ConvertibleToMask<M> + PartialEq + Clone,
        M: Unsigned + BitXorAssign + PrimInt,
    {
        let (left, mut right) = self.tee();

        let mut mask = M::zero();
        // Initialize the mask with the first S-1 elements.
        for _ in 0..S {
            if let Some(item) = right.next() {
                mask ^= item.to_mask();
            } else {
                return None;
            }
        }

        left.zip(right).position(|(item_left, item_right)| {
            if mask.count_ones() == S as u32 {
                return true;
            }

            mask ^= item_right.to_mask() ^ item_left.to_mask();

            false
        })
    }

    /// A naive implementation of `find_first_unique_set_index`, which is not optimized.
    /// This is used for testing against the optimized implementation.
    ///
    /// This uses a [`Vec`] to collect each window of elements, and check if the window
    /// already contains the element while doing so. While naive, this is already "optimized"
    /// over [`std::collections::HashSet`] due to its overhead, as well as providing
    /// early return when a duplicate is found.
    #[cfg(test)]
    fn find_first_unique_set_index_naive<const S: usize>(self) -> Option<usize>
    where
        T: PartialEq + Clone,
    {
        let collected = self.collect::<Vec<_>>();

        let index = collected.windows(S).position(|window| {
            window
                .iter()
                .try_fold(Vec::with_capacity(S), |mut acc, item| {
                    if acc.contains(&item) {
                        None
                    } else {
                        acc.push(item);
                        Some(acc)
                    }
                })
                .is_some()
        });

        index
    }
}
