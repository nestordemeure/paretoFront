use crate::{Dominate, ParetoFront};
use thread_local::ThreadLocal;
use std::{cell::UnsafeCell, marker::Send};

/// Represents a Pareto front that can be pushed into concurrently.
/// TODO note on memory use
/// TODO impl basic traits like Debug
#[derive(Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ThreadSafeParetoFront<T: Dominate + Send>
{
    inner_front: ThreadLocal<UnsafeCell<ParetoFront<T>>>
}

impl<T: Dominate + Send> ThreadSafeParetoFront<T>
{
    /// Constructs a new, empty, Thread-safe Pareto front.
    pub fn new() -> Self
    {
        Self { inner_front: ThreadLocal::new() }
    }

    pub fn push(&self, new_element: T) -> bool
    {
        // gets a mutable *pointer* to the Pareto front bellonging to the current thread
        let front_ptr = self.inner_front.get_or_default().get();
        // safe to mutate because only one thread can access a thread-local front
        // NOTE: this has been validated with a RefCell
        let front = unsafe { &mut *front_ptr };
        // push in the Pareto front
        front.push(new_element)
    }

    pub fn into_sequential(self) -> ParetoFront<T>
    {
        self.inner_front
            .into_iter()
            .map(|r| r.into_inner()) // remove UnsafeCells
            .reduce(|f1, f2| {
                // accumulates in the larger front
                let (mut f1, f2) = if f1.len() > f2.len() { (f1, f2) } else { (f2, f1) };
                f1.extend(f2);
                f1
            })
            .unwrap_or_default() // returns the empty front in the absence of front
    }
}
