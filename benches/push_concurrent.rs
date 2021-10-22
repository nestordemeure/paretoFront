// benchmarking lib
use criterion::{criterion_group, criterion_main, Criterion};
// element type to do our benchmarks on
mod pareto_element;
use pareto_element::ParetoElementCircle as ParetoElement;
// paralelism
use rayon::prelude::*;
// pareto front
use pareto_front::ParetoFront;
use pareto_front::ThreadSafeParetoFront;

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
fn generate_front_threadlocal(data: &[ParetoElement]) -> ParetoFront<ParetoElement>
{
    let concurrent_front = ThreadSafeParetoFront::new();
    data.par_iter().for_each(|x| {
                       concurrent_front.push(*x);
                   });
    concurrent_front.into_sequential()
}

fn generate_front_threadlocal_unreduced(data: &[ParetoElement]) -> ThreadSafeParetoFront<ParetoElement>
{
    let concurrent_front = ThreadSafeParetoFront::new();
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
    let data = ParetoElement::sample_n(5000, seed);
    // compares various functions
    let mut group = c.benchmark_group("compare_push_concurrent_5000");
    group.bench_function("push", |b| b.iter(|| generate_front(&data)));
    group.bench_function("push_threadsafe", |b| b.iter(|| generate_front_threadlocal(&data)));
    group.bench_function("push_unreduced", |b| b.iter(|| generate_front_threadlocal_unreduced(&data)));
    group.finish();
}

criterion_group!(benches, comparison_benchmark);
criterion_main!(benches);
