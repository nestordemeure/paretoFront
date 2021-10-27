/// Used to define a pseudo-ordering for multi-dimensional optimization.
pub trait Dominate
{
    /// Returns `true` if we are better (which might be superior or inferior depending on the specification) than `x` along all dimenssions.
    /// By convention, it usually returns `false` if `x` is equal to `self`.
    ///
    /// ```rust
    /// # use pareto_front::{Dominate, ParetoFront};
    /// #
    /// /// type that will be pushed in the Pareto front
    /// #[derive(PartialEq)]
    /// struct ParetoElement
    /// {
    ///     cost: usize, // to be minimized
    ///     quality: f32, // to be maximized
    /// }
    ///
    /// /// implement the `Dominate` trait so that the elements can be pushed into the front
    /// impl Dominate for ParetoElement
    /// {
    ///     /// returns `true` is `self` is better than `x` on all fields that matter to us
    ///     fn dominate(&self, x: &Self) -> bool
    ///     {
    ///         (self.cost <= x.cost) && (self.quality >= x.quality) && (self != x)
    ///     }
    /// }
    /// ```
    fn dominate(&self, x: &Self) -> bool;
}
