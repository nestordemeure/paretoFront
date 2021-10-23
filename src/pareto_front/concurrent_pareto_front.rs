use crate::{Dominate, ParetoFront};
use thread_local::ThreadLocal;
use std::{cell::UnsafeCell, marker::Send};

/// Represents a Pareto front that can be pushed into concurrently.
///
/// As this implementation is based on thread-local fronts,
/// one would get better performance by having explicitely one `ParetoFront` per thread
/// and merging them when needed.
///
/// We expect this implementation to use approximately `O(t*n)` memory
/// where `t` is the number of threads used
/// and `n` is the size of the corresponding sequential Pareto front.
#[derive(Default, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ConcurrentParetoFront<T: Dominate + Send>
{
    inner_front: ThreadLocal<UnsafeCell<ParetoFront<T>>>
}

impl<T: Dominate + Send> ConcurrentParetoFront<T>
{
    /// Constructs a new, empty, concurrent Pareto front.
    pub fn new() -> Self
    {
        ConcurrentParetoFront { inner_front: ThreadLocal::new() }
    }

    /// Adds `new_element` to the Pareto front.
    /// Returns `true` if the element *might be* in the Pareto front.
    /// Returns `false` if the element was dominated and, thus, not added to the front.
    ///
    /// This operation has `O(n/t)` complexity
    /// where `n` is the number of elements currently in the Pareto front
    /// and `t` the number of threads used
    /// but is optimized to favour early stopping and cache friendly.
    pub fn push(&self, new_element: T) -> bool
    {
        // gets a mutable *pointer* to the Pareto front associated with the current thread
        let front_ptr = self.inner_front.get_or_default().get();
        // converts the pointer into a mutable reference
        // Note: safe because only one thread can access a thread-local front
        //       this has been validated with a RefCell
        let front = unsafe { &mut *front_ptr };
        // push the new element in the Pareto front
        front.push(new_element)
    }

    /// Turns the concurrent Pareto front into a, sequential, `ParetoFront`.
    ///
    /// This operation has complexity `O(t*n)`
    /// where `t` is the number of threads used
    /// and `n` is the size of the Pareto front.
    ///
    /// Note that this operation does *not* use any interior paralelism.
    pub fn into_sequential(self) -> ParetoFront<T>
    {
        // NOTE: this could be turned into a parallel reduce
        //       but, tests with `rayon` did not bring any significant speed benefits
        //       however, paralelism might become beneficial on a large (16+) number of cores
        self.inner_front
            .into_iter()
            .map(|r| r.into_inner()) // remove UnsafeCells
            .reduce(|mut front_acc, front| {
                // merge all fronts into one
                front_acc.merge(front);
                front_acc
            })
            .unwrap_or_default() // returns an empty front if there was no thread-local front
    }
}

impl<T: Dominate + Send> Into<Vec<T>> for ConcurrentParetoFront<T>
{
    /// Converts the concurrent Pareto front into a vector.
    /// This operation has the complexity of `into_sequential`.
    fn into(self) -> Vec<T>
    {
        self.into_sequential().into()
    }
}

impl<T: Dominate + Send> From<ConcurrentParetoFront<T>> for ParetoFront<T>
{
    /// Converts the concurrent Pareto front into a `ParetoFront`.
    /// This operation has the complexity of `into_sequential`.
    fn from(front: ConcurrentParetoFront<T>) -> ParetoFront<T>
    {
        front.into_sequential()
    }
}

impl<T: Dominate + Send> From<ParetoFront<T>> for ConcurrentParetoFront<T>
{
    /// Converts a `ParetoFront` into a concurrent Pareto front.
    /// this operation has complexity `O(1)`.
    fn from(front: ParetoFront<T>) -> Self
    {
        // creates new, empty, concurrent Pareto front
        let result = ConcurrentParetoFront::new();
        // tries to get a thread-local pareto front
        // as the front is empty, it triggers the call to front
        result.inner_front.get_or(|| UnsafeCell::new(front));
        // returns result
        result
    }
}

impl<T: Dominate + Send> IntoIterator for ConcurrentParetoFront<T>
{
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    /// Creates an iterator from a `ConcurrentParetoFront`.
    /// This operation has the complexity of `into_sequential`.
    fn into_iter(self) -> Self::IntoIter
    {
        self.into_sequential().into_iter()
    }
}

impl<T: Dominate + Send> FromIterator<T> for ConcurrentParetoFront<T>
{
    /// Implements the `FromIterator` trait to enable the collection of an iterator into a `ConcurrentParetoFront`.
    ///
    /// Note that, while this operation is slightly cheaper than a serie of sequential push into a `ConcurrentParetoFront`,
    /// it does *not* use any interior paralelism.
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self
    {
        let mut front = ConcurrentParetoFront::new();
        front.extend(iter); // we reuse the implementation of extend
        front
    }
}

impl<T: Dominate + Send> Extend<T> for ConcurrentParetoFront<T>
{
    /// Implements the `Extend` trait to extend a `ConcurrentParetoFront` with the content of an iterator.
    ///
    /// Note that, while this operation is slightly cheaper than a serie of sequential push into a `ConcurrentParetoFront`,
    /// it does *not* use any interior paralelism.
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I)
    {
        // gets a mutable *pointer* to the Pareto front associated with the current thread
        let front_ptr = self.inner_front.get_or_default().get();
        // converts the pointer into a mutable reference
        // Note: safe because only one thread can access a thread-local front
        //       this has been validated with a RefCell
        let front = unsafe { &mut *front_ptr };
        // push the new elements in the Pareto front
        front.extend(iter)
    }
}
