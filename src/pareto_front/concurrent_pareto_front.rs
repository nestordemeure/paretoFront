use crate::{Dominate, ParetoFront};
use thread_local::ThreadLocal;
use std::{cell::UnsafeCell, marker::Send};

/// Represents a Pareto front that can be pushed into concurrently.
/// TODO note on memory use
/// TODO impl basic traits like Debug
#[derive(Default)]
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

    /// Turns the concurrent Pareto front into a, sequential, `ParetoFront`
    ///
    /// This operation has complexity `O(t*n)` where `t` is the number of thread used
    /// and `n` is the size fo the Pareto front.
    ///
    /// Note that this operation does *not* use any interior paralelism.
    pub fn into_sequential(self) -> ParetoFront<T>
    {
        // NOTE: this could be turned into a parallel reduce
        //       but, a test with `rayon` did not bring any significant speed benefits
        //       however, paralelism might become beneficial on a large (16+) number of cores
        self.inner_front
            .into_iter()
            .map(|r| r.into_inner()) // remove UnsafeCells
            .reduce(|mut front_acc, front| {
                // merge all fronts into one
                front_acc.merge(front);
                front_acc
            })
            .unwrap_or_default() // returns an empty front if no thread ever added to the front
    }
}
