//! Element type to be used in tests and bench
use pareto_front::Dominate;
use rand::{Rng, SeedableRng, rngs::StdRng};
use rand_distr::{Distribution, Uniform};

/// element to test the pareto front
/// this, 2D, element type is a very common use case
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ParetoElementCircle
{
    x: f64,
    y: f64
}

impl Dominate for ParetoElementCircle
{
    /// function to determine wether an element dominates another element
    fn dominate(&self, x: &Self) -> bool
    {
        (self.x <= x.x) && // minimize x
        (self.y <= x.y) &&  // minimize y
        (self != x) // not equal
    }
}

impl ParetoElementCircle
{
    /// creates a random element using the given random number generator
    /// alements are within a circle
    pub fn sample<R: Rng + ?Sized>(rng: &mut R) -> Self
    {
        let distribution = Uniform::new(0., 1.);
        let mut x: f64 = distribution.sample(rng);
        let mut y: f64 = distribution.sample(rng);
        while (1. - x).hypot(1. - y) > 1.
        {
            x = distribution.sample(rng);
            y = distribution.sample(rng);
        }
        Self { x, y }
    }

    /// creates the given number of elements and put them in a slice
    /// uses the given seed for reproducibility
    pub fn sample_n(n: usize, seed: u64) -> Vec<Self>
    {
        let mut rng = StdRng::seed_from_u64(seed);
        (0..n).map(|_| Self::sample(&mut rng)).collect()
    }
}
