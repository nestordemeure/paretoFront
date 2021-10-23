use rand::{Rng, SeedableRng, rngs::StdRng};
use pareto_front::Dominate;

/// type of the elemnts to be inserted in the front
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ParetoElement
{
    pub cost: usize,
    pub quality: u8,
    pub score: i64
}

/// implement the `Dominate` trait
impl Dominate for ParetoElement
{
    fn dominate(&self, x: &Self) -> bool
    {
        (self.cost <= x.cost) && (self.quality >= x.quality) && (self.score >= x.score) && (self != x)
    }
}

impl ParetoElement
{
    /// creates a fully random element using the given random number generator
    pub fn sample<R: Rng + ?Sized>(rng: &mut R) -> Self
    {
        Self { cost: rng.gen(), quality: rng.gen(), score: rng.gen() }
    }

    /// creates the given number of elements and put them in a slice
    /// uses the given seed for reproducibility
    pub fn sample_n(n: usize, seed: u64) -> Vec<Self>
    {
        let mut rng = StdRng::seed_from_u64(seed);
        (0..n).map(|_| Self::sample(&mut rng)).collect()
    }
}
