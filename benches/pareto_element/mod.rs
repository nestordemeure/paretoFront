// fully random element
mod random;
pub use random::ParetoElementRandom;
// elements drawed from a 2D circle
mod circle;
pub use circle::ParetoElementCircle;
// elements drawed from a 5D circle, a worst case scenario
mod circle5D;
pub use circle5D::ParetoElementCircle5D;
