//! Element types that used to run the `ParetoFront` benchmarks
#![allow(dead_code)]
mod random;
pub use random::ParetoElementRandom;
mod circle;
pub use circle::ParetoElementCircle;
mod circle5d;
pub use circle5d::ParetoElementCircle5D;
