//! Element type to be used in tests and bench
use pareto_front::Dominate;
use rand::{Rng, SeedableRng, rngs::StdRng};

/// test element type
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ParetoElement
{
    cost: usize,
    quality: f32,
    score: i64
}

impl Dominate for ParetoElement
{
    /// function to determine wether an element dominates another element
    fn dominate(&self, x: &Self) -> bool
    {
        (self.cost <= x.cost) && // minimize cost
        (self.quality >= x.quality) &&  // maximize quality
        (self.score >= x.score) &&  // maximize score
        (self != x) // not equal
    }
}

impl ParetoElement
{
    /// creates a fully random element using the given random number generator
    pub fn sample<R: Rng + ?Sized>(rng: &mut R) -> Self
    {
        ParetoElement { cost: rng.gen(), quality: rng.gen(), score: rng.gen() }
    }

    /// creates the given number of elements and put them in a slice
    /// uses the given seed for reproducibility
    pub fn sample_n(n: usize, seed: u64) -> Vec<ParetoElement>
    {
        let mut rng = StdRng::seed_from_u64(seed);
        (0..n).map(|_| Self::sample(&mut rng)).collect()
    }
}
