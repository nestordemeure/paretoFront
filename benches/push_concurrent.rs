// benchmarking lib
use criterion::{criterion_group, criterion_main, Criterion};
// element type to do our benchmarks on
mod pareto_element;
use pareto_element::ParetoElementCircle as ParetoElement;
// pareto front
use pareto_front::ParetoFront;
// paralelism
#[cfg(feature = "pareto_front_concurrent")]
use rayon::prelude::*;
#[cfg(feature = "pareto_front_concurrent")]
use pareto_front::ConcurrentParetoFront;

/// inserts all the element from data into a pareto front using the `push` function
fn generate_front(data: &[ParetoElement]) -> ParetoFront<ParetoElement>
{
    let mut front = ParetoFront::new();
    data.iter().for_each(|x| {
                   front.push(*x);
               });
    front
}

/// insert concurrently in thread local copies of the thread that are merged after the fact
#[cfg(feature = "pareto_front_concurrent")]
fn generate_front_concurrent(data: &[ParetoElement]) -> ParetoFront<ParetoElement>
{
    let concurrent_front = ConcurrentParetoFront::new();
    data.par_iter().for_each(|x| {
                       concurrent_front.push(*x);
                   });
    concurrent_front.into_sequential()
}

/// same thing but without `into_sequential` to evaluate its cost
#[cfg(feature = "pareto_front_concurrent")]
fn generate_front_concurrent_unreduced(data: &[ParetoElement]) -> ConcurrentParetoFront<ParetoElement>
{
    let concurrent_front = ConcurrentParetoFront::new();
    data.par_iter().for_each(|x| {
                       concurrent_front.push(*x);
                   });
    concurrent_front
}

/// compares two implementation of the `push` function
fn comparison_benchmark(c: &mut Criterion)
{
    // data used for the bench
    let seed = 42;
    let data = ParetoElement::sample_n(5000000, seed);
    // compares various functions
    let mut group = c.benchmark_group("compare_push_concurrent_5000000");
    group.bench_function("push_sequential", |b| b.iter(|| generate_front(&data)));
    #[cfg(feature = "pareto_front_concurrent")]
    group.bench_function("push_concurrent", |b| b.iter(|| generate_front_concurrent(&data)));
    #[cfg(feature = "pareto_front_concurrent")]
    group.bench_function("push_concurrent_unreduced", |b| {
             b.iter(|| generate_front_concurrent_unreduced(&data))
         });
    group.finish();
}

criterion_group!(benches, comparison_benchmark);
criterion_main!(benches);
