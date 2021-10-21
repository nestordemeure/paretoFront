//! Element type to be used in tests and bench
use std::convert::TryInto;
use pareto_front::Dominate;
use rand::{Rng, SeedableRng, rngs::StdRng};
use rand_distr::{Distribution, Uniform};

/// elements to test the pareto front
/// elements are in a 5D circle, due to the high dimenssion we expect a large number of them to be on the pareto front
/// that should be a *worst-case* situation
/// (unlikely to happen in real use case as, in such a high dimenssion, the pareto front is close to meaningless)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ParetoElementCircle5D
{
    coordinates: [f64; 5]
}

impl Dominate for ParetoElementCircle5D
{
    /// function to determine wether an element dominates another element
    fn dominate(&self, x: &Self) -> bool
    {
        // minimize all coordinates
        self.coordinates.iter().zip(x.coordinates.iter())
                        .all(|(sc,xc)| sc <= xc)
        // not equal
        && (self != x)
    }
}

impl ParetoElementCircle5D
{
    /// creates a random element using the given random number generator
    /// alements are within a circle
    pub fn sample<R: Rng + ?Sized>(mut rng: &mut R) -> Self
    {
        let distribution = Uniform::new(0., 1.);
        let mut coordinates: Vec<f64> = distribution.sample_iter(&mut rng).take(5).collect();
        let mut radius = coordinates.iter().map(|c| (1. - c) * (1. - c)).sum::<f64>();
        while radius > 1.
        {
            coordinates = distribution.sample_iter(&mut rng).take(5).collect();
            radius = coordinates.iter().map(|c| (1. - c) * (1. - c)).sum::<f64>();
        }
        Self { coordinates: coordinates.try_into().unwrap() }
    }

    /// creates the given number of elements and put them in a slice
    /// uses the given seed for reproducibility
    pub fn sample_n(n: usize, seed: u64) -> Vec<Self>
    {
        let mut rng = StdRng::seed_from_u64(seed);
        (0..n).map(|_| Self::sample(&mut rng)).collect()
    }
}
