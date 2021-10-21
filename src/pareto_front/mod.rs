mod dominate;
pub use dominate::Dominate;
use std::slice::{Iter, IterMut};
use std::iter::FromIterator;

/// Represents a Pareto front.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ParetoFront<T: Dominate>
{
    front: Vec<T>
}

impl<T: Dominate> ParetoFront<T>
{
    /// Constructs a new, empty, Pareto front.
    pub fn new() -> Self
    {
        return ParetoFront { front: Vec::new() };
    }

    /// Removes all elements in the front that are dominated by `new_element`,
    /// starting at index `index_start`.
    fn _remove_dominated_starting_at(&mut self, new_element: &T, index_start: usize)
    {
        // lists all elements dominated by `new_element`, starting at index `index_start`
        let mut index_dominated_elements = Vec::new();
        for (index, element) in self.front.iter().enumerate().skip(index_start)
        {
            if new_element.dominate(element)
            {
                index_dominated_elements.push(index);
            }
        }

        // removes the elements at the listed indexes
        // taking into acount that each removed index will shift all the following indexes
        // NOTE: reversing the iterator removes the need for `nb_elements_removed` but is slightly slower in my tests
        for (nb_elements_removed, index) in index_dominated_elements.into_iter().enumerate()
        {
            self.front.swap_remove(index - nb_elements_removed);
        }
    }

    /// Adds `new_element` to the Pareto front.
    /// Returns `true` if the element is now in the Pareto front.
    /// Returns `false` if the element was dominated and, thus, not added to the front.
    ///
    /// This operation as `O(n)` complexity (where `n` is the number of elements currently in the Pareto front)
    /// but is optimized to favour early stopping and cache friendly.
    ///
    /// This operation might *not* preserve the ordering of the elements in the front.
    ///
    /// ```rust
    /// # use pareto_front::{Dominate, ParetoFront};
    /// #
    /// # /// type that will be pushed in the Pareto front
    /// # #[derive(PartialEq)]
    /// # struct ParetoElement
    /// # {
    /// #    cost: usize, // to be minimized
    /// #    quality: f32, // to be maximized
    /// # }
    /// #
    /// # /// implement the `Dominate` trait so that the elements can be pushed into the front
    /// # impl Dominate for ParetoElement
    /// # {
    /// #    /// returns `true` is `self` is better than `x` on all fields that matter to us
    /// #    fn dominate(&self, x: &Self) -> bool
    /// #    {
    /// #        (self.cost <= x.cost) && (self.quality >= x.quality) && (self != x)
    /// #    }
    /// # }
    /// #
    /// # // data to be put in the front
    /// # let x = ParetoElement { cost: 35, quality: 0.5 };
    /// #
    /// // a Pareto front
    /// let mut front = ParetoFront::new();
    ///
    /// // inserts in the Pareto front
    /// let is_pareto_optimal = front.push(x);
    /// ```
    pub fn push(&mut self, new_element: T) -> bool
    {
        // for all elements of the pareto front, check whether they are dominated or dominate `new_element`
        for (index, element) in self.front.iter().enumerate()
        {
            if element.dominate(&new_element)
            {
                // `new_element` is dominated by `element`, it is thus not part of the Pareto front
                // swap `element` with the previous element in order to percolate the best elements to the top
                // NOTE: in my benchmarks this brings clear performance benefits by putting "killer" elements first
                if index > 0
                {
                    self.front.swap(index, index - 1);
                }
                return false;
            }
            else if new_element.dominate(element)
            {
                // `new_element` dominates `element`, it is thus part of the Pareto front
                // look at the rest of the Pareto front to remove any further element that is dominated
                self._remove_dominated_starting_at(&new_element, index + 1);
                // replace `element` with `new_element`
                self.front[index] = new_element;
                return true;
            }
        }

        // `new_element` has not been dominated, it is thus part of the Pareto front
        self.front.push(new_element);
        return true;
    }

    /// Extracts a slice containing the entire Pareto front.
    pub fn as_slice(&self) -> &[T]
    {
        self.front.as_slice()
    }

    /// Returns the number of elements currently in the Pareto front.
    pub fn len(&self) -> usize
    {
        self.front.len()
    }

    /// Returns an iterator over the Pareto front.
    pub fn iter(&self) -> Iter<T>
    {
        self.front.iter()
    }

    /// Returns an iterator that allows modifying each value.
    pub fn iter_mut(&mut self) -> IterMut<T>
    {
        self.front.iter_mut()
    }
}

impl<T: Dominate> Into<Vec<T>> for ParetoFront<T>
{
    /// Converts the Pareto front into a vector.
    /// This operation is free as the underlying datastructure is a vector.
    fn into(self) -> Vec<T>
    {
        self.front
    }
}

impl<T: Dominate> IntoIterator for ParetoFront<T>
{
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    /// Creates an iterator from a `ParetoFront`.
    fn into_iter(self) -> Self::IntoIter
    {
        self.front.into_iter()
    }
}

impl<T: Dominate> FromIterator<T> for ParetoFront<T>
{
    /// Implements the `FromIterator` trait to enable the collection of an iterator into a `ParetoFront`.
    ///
    /// ```rust
    /// # use pareto_front::{Dominate, ParetoFront};
    /// #
    /// # /// type that will be pushed in the Pareto front
    /// # #[derive(PartialEq)]
    /// # struct ParetoElement
    /// # {
    /// #    cost: usize, // to be minimized
    /// #    quality: f32, // to be maximized
    /// # }
    /// #
    /// # /// implement the `Dominate` trait so that the elements can be pushed into the front
    /// # impl Dominate for ParetoElement
    /// # {
    /// #    /// returns `true` is `self` is better than `x` on all fields that matter to us
    /// #    fn dominate(&self, x: &Self) -> bool
    /// #    {
    /// #        (self.cost <= x.cost) && (self.quality >= x.quality) && (self != x)
    /// #    }
    /// # }
    /// #
    /// # // data to be put in the front
    /// # let x = ParetoElement { cost: 35, quality: 0.5 };
    /// # let y = ParetoElement { cost: 35, quality: 0.5 };
    /// # let z = ParetoElement { cost: 35, quality: 0.5 };
    /// #
    /// // builds a Pareto front from an iterator
    /// let front : ParetoFront<_> = vec![x, y, z].into_iter().collect();
    /// ```
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

impl<T: Dominate> Extend<T> for ParetoFront<T>
{
    /// Implements the `Extend` trait to extend a `ParetoFront` with the content of an iterator.
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I)
    {
        for x in iter
        {
            self.push(x);
        }
    }
}
