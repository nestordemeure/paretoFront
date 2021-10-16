mod dominate;
pub use dominate::Dominate;
use std::slice::{Iter, IterMut};
use std::iter::FromIterator;

/// represents a pareto front
#[derive(Clone, Debug, Default)]
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
                if index > 0
                {
                    self.front.swap(index, index - 1);
                }
                return false;
            }
            else if x.dominate(element)
            {
                // x dominated an element and is thus part of the pareto front
                indexes_dominated_elements.push(index);
            }
        }

        // removes the element that have been dominated
        for (nb_id_already_removed, index) in indexes_dominated_elements.into_iter().enumerate()
        {
            // we correct the index by taking into account the number of indexes that have already been removed
            // using the fact that they are strictly increasing
            self.front.swap_remove(index - nb_id_already_removed);
        }

        // x has not been dominated, it is thus part of the pareto front
        self.front.push(x);

        return true;
    }

    /// returns the pareto front as a slice
    pub fn front(&self) -> &[T]
    {
        self.front.as_slice()
    }

    /// returns the number of elements in the front
    pub fn len(&self) -> usize
    {
        self.front.len()
    }

    /// returns an iterator
    pub fn iter(&self) -> Iter<T>
    {
        self.front.iter()
    }

    /// returns an iterator that allows modifying each value
    pub fn iter_mut(&mut self) -> IterMut<T>
    {
        self.front.iter_mut()
    }
}

impl<T: Dominate> IntoIterator for ParetoFront<T>
{
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    /// creates an iterator from value
    fn into_iter(self) -> Self::IntoIter
    {
        self.front.into_iter()
    }
}

/// implement Into<Vec> trait to let user easily convert the collection into a vector
impl<T: Dominate> Into<Vec<T>> for ParetoFront<T>
{
    /// this is free as the underlying datastructure is a vector
    fn into(self) -> Vec<T>
    {
        self.front
    }
}

/// implements the FromIterator trait to enable the collection of an iterator into a front
impl<T: Dominate> FromIterator<T> for ParetoFront<T>
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self
    {
        let mut front = ParetoFront::new();

        for x in iter
        {
            front.push(x);
        }

        front
    }
}

/// implements the Extend trait to let iterator add to an existing front
impl<T: Dominate> Extend<T> for ParetoFront<T>
{
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I)
    {
        for x in iter
        {
            self.push(x);
        }
    }
}
