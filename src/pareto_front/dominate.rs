/// Used to define a pseudo-ordering for multi-dimensional optimization
pub trait Dominate
{
    /// Returns `true` if we are better (which might be superior or inferior depending on the specification) than `x` along all dimenssions.
    /// By default, returns `false` if `x` is equal to `self`
    fn dominate(&self, x: &Self) -> bool;
}
