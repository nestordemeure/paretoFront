pub use crate::{Dominate, ParetoFront};
use thread_local::ThreadLocal;
use std::{cell::RefCell, marker::Send};

/// Represents a Pareto front that can be pushed into concurrently.
/// TODO note on memory use
/// TODO impl basic traits like Debug
#[derive(Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ThreadSafeParetoFront<T: Dominate + Send>
{
    inner_front: ThreadLocal<RefCell<ParetoFront<T>>>
}

// TODO remove need for Default trait
impl<T: Dominate + Send> ThreadSafeParetoFront<T>
{
    /// Constructs a new, empty, Thread-safe Pareto front.
    pub fn new() -> Self
    {
        return Self { inner_front: ThreadLocal::new() };
    }

    pub fn push(&self, new_element: T) -> bool
    {
        // gets a mutable reference to the Pareto front bellonging to the current thread
        let mut front = self.inner_front.get_or_default().borrow_mut();
        // push in the Pareto front
        front.push(new_element)
    }

    pub fn into_sequential(self) -> ParetoFront<T>
    {
        self.inner_front
            .into_iter()
            .map(|r| r.into_inner()) // remove refcells
            .reduce(|f1, f2| {
                // accumulates in the larger front
                let (mut f1, f2) = if f1.len() > f2.len() { (f1, f2) } else { (f2, f1) };
                f1.extend(f2);
                f1
            })
            .unwrap_or_default() // returns the empty front in the absence of front
    }
}
