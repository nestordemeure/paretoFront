# Pareto Front ([crates.io](https://crates.io/crates/pareto_front))

The `pareto_front` crate is a Rust library to build a [Pareto front](https://en.wikipedia.org/wiki/Pareto_front) incrementaly.

This is particularly useful in multi-objectives optimization where, instead of having a single maximum that one can easily keep track off, one might want to keep track of various trade-offs, none of which is best on all axis, found during the optimization.

This crate tries to be small yet *really fast* and correct.

## Functionalities

This crate gives you access to the `ParetoFront` type which can be created (empty or from an iterator), updated by adding new candidates (using the `push` or the `extend` method) and converted into an iterator, a slice or a vector.

The `pareto_front_concurrent` feature unlocks the `ConcurrentParetoFront` type which can be used to build a Pareto front inside a parallel algorithm without needing to put a lock around a `ParetoFront`.

The `pareto_front_serde` feature lets you serialize and deserialize the `ParetoFront` type using [serde](https://serde.rs/).

## Usage

Elements to be inserted in the Pareto front should implement the `Dominate` trait:

```rust
use pareto_front::{Dominate, ParetoFront};

/// type that will be pushed in the Pareto front
#[derive(PartialEq)]
struct ParetoElement
{
    cost: usize, // to be minimized
    quality: f32, // to be maximized
}

/// implement the `Dominate` trait so that the elements can be pushed into the front
impl Dominate for ParetoElement
{
    /// returns `true` is `self` is better than `x` on all fields that matter to us 
    fn dominate(&self, x: &Self) -> bool
    {
        (self.cost <= x.cost) && (self.quality >= x.quality) && (self != x)
    }
}
```

New elements can be added to a Pareto front using the `push` method (one can also `collect` an iterator into a Pareto front):

```rust
// data to be put in the front
let x = ParetoElement { cost: 35, quality: 0.5 };
let y = ParetoElement { cost: 350, quality: 0.05 };
let z = ParetoElement { cost: 5, quality: 0.25 };

// insertions in the Pareto front
let mut front = ParetoFront::new();
front.push(x);
front.push(y);

// note that `push` returns a boolean to tell you if the point you just inserted is part of the current Pareto front
let z_is_pareto_optimal = front.push(z);
```

The resulting Pareto front can be converted into an iterator, a slice or a vector.
