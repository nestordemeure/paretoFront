/// Used to define a pseudo ordering for multi-dimensional optimization
pub trait Dominate
{
    /// Returns `true` if we are better (superior or inferior depending on the specification) than `x` along all dimenssion
    /// By default, returns `false` if `x` is equal to us
    fn dominate(&self, x: &Self) -> bool;
}
