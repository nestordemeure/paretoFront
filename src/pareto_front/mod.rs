mod dominate;
pub use dominate::Dominate;
use std::slice::{Iter, IterMut};

/// represents a pareto front
#[derive(Debug)]
pub struct ParetoFront<T: Dominate>
{
    front: Vec<T>
}

impl<T: Dominate> ParetoFront<T>
{
    /// constructs a new, empty, pareto front
    pub fn new() -> Self
    {
        return ParetoFront { front: Vec::new() };
    }

    /// adds an element to the front
    /// returns `true` if the element was in the pareto front
    /// returns `false` if the element was dominated and, thus, not added
    pub fn push(&mut self, x: T) -> bool
    {
        let mut indexes_dominated_elements = Vec::new();

        for (index, element) in self.front.iter().enumerate()
        {
            if element.dominate(&x)
            {
                // x was not part of the pareto front
                // swap element with the previous element in order to percolate the best elements to the top
                /*if index > 0
                {
                    self.front.swap(index, index - 1);
                }*/
                return false;
            }
            else if x.dominate(element)
            {
                // x dominated an element and is thus part of the pareto front
                indexes_dominated_elements.push(index);
            }
        }

        // x has not been dominated, it is thus part of the pareto front
        self.front.push(x);
        for index in indexes_dominated_elements
        {
            self.front.swap_remove(index);
        }

        return true;
    }

    /// returns the number of elements in the front
    pub fn count(&self) -> usize
    {
        return self.front.len();
    }

    /// returns an iterator
    pub fn iter(&self) -> Iter<T>
    {
        return self.front.iter();
    }

    /// returns an iterator that allows modifying each value
    pub fn iter_mut(&mut self) -> IterMut<T>
    {
        return self.front.iter_mut();
    }

    /// creates an iterator from value
    pub fn into_iter(self) -> std::vec::IntoIter<T>
    {
        return self.front.into_iter();
    }
}
