use std::ops::BitXorAssign;

use num_traits::{PrimInt, Unsigned};

use super::ConvertibleToMask;

#[cfg(test)]
use timeit::timeit_loops;

/// A trait for finding the first unique set of elements in a collection.
pub trait FindFirstUniqueSet<T> {
    /// Find the first unique set of elements in a collection.
    fn find_first_unique_set_index<const S: usize, M>(self) -> Option<usize>
    where
        M: Unsigned + BitXorAssign + PrimInt,
        T: ConvertibleToMask<M> + PartialEq + Clone;

    #[cfg(test)]
    /// A naive implementation of `find_first_unique_set_index`, which is not optimized.
    /// This is used for testing against the optimized implementation.
    fn find_first_unique_set_index_naive<const S: usize>(self) -> Option<usize>
    where
        T: PartialEq + Clone;
}

/// A trait for timing the optimized and naive implementations of finding the first unique set.
///
/// This trait is only available in tests, and requires the [`Iterator`] to be [`Clone`]; which
/// is a rare requirement for iterators. Typically used with [`str::chars`].
#[cfg(test)]
pub trait FindFirstUniqueSetTimed<T>: FindFirstUniqueSet<T>
where
    Self: Sized + Clone,
{
    #[cfg(test)]
    fn time_first_unique_set_index<const S: usize, M>(self, count: usize) -> f64
    where
        M: Unsigned + BitXorAssign + PrimInt,
        T: ConvertibleToMask<M> + PartialEq + Clone,
    {
        timeit_loops!(count, {
            self.clone().find_first_unique_set_index::<S, M>();
        })
    }

    #[cfg(test)]
    fn time_first_unique_set_index_naive<const S: usize>(self, count: usize) -> f64
    where
        T: PartialEq + Clone,
    {
        timeit_loops!(count, {
            self.clone().find_first_unique_set_index_naive::<S>();
        })
    }
}

#[cfg(test)]
impl<U, T> FindFirstUniqueSetTimed<T> for U where U: FindFirstUniqueSet<T> + Clone + Sized {}
