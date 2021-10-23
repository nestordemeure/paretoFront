use crate::Dominate;
use std::slice::Iter;
use std::iter::FromIterator;

/// Represents a Pareto front.
#[derive(Clone, Debug)]
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
        // in reverse order to take into acount that each removed index shift all the following indexes
        for index in index_dominated_elements.into_iter().rev()
        {
            self.front.swap_remove(index);
        }
    }

    /// Removes all the elements in the Pareto front that are dominated by `new_element`.
    /// Returns `true` if `new_element` should be in the Pareto front.
    /// Returns `false` if `new_element` was dominated and, thus, shouldn't be added to the front.
    ///
    /// This operation has `O(n)` complexity (where `n` is the number of elements currently in the Pareto front)
    /// but is optimized to favour early stopping and cache friendly.
    ///
    /// This operation might *not* preserve the ordering of the elements in the front.
    fn _remove_dominated(&mut self, new_element: &T) -> bool
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
                self.front.swap_remove(index);
                // looks at the rest of the Pareto front to remove any further element that are dominated
                self._remove_dominated_starting_at(&new_element, index);
                return true;
            }
        }

        // `new_element` has not been dominated, it is thus part of the Pareto front
        return true;
    }

    /// Adds `new_element` to the Pareto front.
    /// Returns `true` if the element is now in the Pareto front.
    /// Returns `false` if the element was dominated and, thus, not added to the front.
    ///
    /// This operation has `O(n)` complexity (where `n` is the number of elements currently in the Pareto front)
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
        // removes dominated elements from the front and checks whether `new_element` should be added
        let is_pareto_optimal = self._remove_dominated(&new_element);
        // adds `new_element` if needed
        if is_pareto_optimal
        {
            self.front.push(new_element);
        }
        return is_pareto_optimal;
    }

    /// Adds the content of `pareto_front` to the Pareto front.
    ///
    /// This operation has `O(n*m)` complexity
    /// where `n` is the number of elements in `self`
    /// and `m` is the number of elements in `pareto_front`.
    pub fn merge(&mut self, pareto_front: ParetoFront<T>)
    {
        // set the largest front aside
        let mut largest_front = pareto_front.front;
        if largest_front.len() < self.front.len()
        {
            std::mem::swap(&mut self.front, &mut largest_front);
        }
        // for all the elements in the largest front, remove dominated elements from the smallest front
        // keep only the elements that should be in the Pareto front
        largest_front = largest_front.into_iter().filter(|x| self._remove_dominated(x)).collect();
        // extends the largest front with the content of the smallest front
        // and make it our front
        std::mem::swap(&mut self.front, &mut largest_front);
        self.front.extend(largest_front);
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
}

impl<T: Dominate> Default for ParetoFront<T>
{
    /// Default value.
    fn default() -> Self
    {
        // Manually implemented so as to not require `T` to implement `Default`.
        Self::new()
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
        // Note: I tried a divide and conquer type of approach
        //       (creating a new pareto front from `iter` and merging it)
        //       but it was slightly slower for all problem sizes
        for x in iter
        {
            self.push(x);
        }
    }
}
