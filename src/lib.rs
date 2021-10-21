//! `pareto_front`
//!
//!
//!  Elements to be inserted in the Pareto front should implement the `Dominate` trait:
//!
//! ```rust
//! use pareto_front::{Dominate, ParetoFront};
//!
//! /// type that will be pushed in the Pareto front
//! #[derive(Debug, PartialEq)]
//! struct ParetoElement
//! {
//!     cost: usize, // to be minimized
//!     quality: f32, // to be maximized
//! }
//!
//! /// implement the `Dominate` trait so that the elements can be pushed into the front
//! impl Dominate for ParetoElement
//! {
//!     /// returns `true` is `self` is better than `x` on all fields that matter to us
//!     fn dominate(&self, x: &Self) -> bool
//!     {
//!         (self.cost <= x.cost) && (self.quality >= x.quality) && (self != x)
//!     }
//! }
//! ```
//!
//! New elements can be added to a Pareto front using the `push` method (one can also `collect` an iterator into a Pareto front):
//!
//! ```rust
//! // data to be put in the front
//! let x = ParetoElement { cost: 35, quality: 0.5 };
//! let y = ParetoElement { cost: 350, quality: 0.05 };
//! let z = ParetoElement { cost: 5, quality: 0.25 };
//!
//! // insertions in the Pareto front
//! let mut front = ParetoFront::new();
//! front.push(x);
//! front.push(y);
//!
//! // note that `push` returns a boolean to tell you if the point you just inserted is part of the current Pareto front
//! let z_is_optimal = front.push(z);
//! ```
//!
//! The resultng Pareto front can be converted into an iterator, a slice or a vector.

#![allow(dead_code)]
mod pareto_front;
pub use pareto_front::Dominate;
pub use pareto_front::ParetoFront;
