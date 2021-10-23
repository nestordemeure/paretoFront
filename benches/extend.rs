// benchmarking lib
use criterion::{criterion_group, criterion_main, Criterion};
// element type to do our benchmarks on
mod pareto_element;
use pareto_element::ParetoElementCircle as ParetoElement;
// pareto front
use pareto_front::ParetoFront;

/// inserts all the element from data into a pareto front using the `extend` function
fn extend_fronts(mut front: ParetoFront<ParetoElement>,
                 data: Vec<ParetoElement>)
                 -> ParetoFront<ParetoElement>
{
    front.extend(data);
    front
}

/*fn extend_fronts2(mut front: ParetoFront<ParetoElement>,
                  data: Vec<ParetoElement>)
                  -> ParetoFront<ParetoElement>
{
    front.extend2(data);
    front
}*/

/// measures the speed of several insertions in a row
fn criterion_benchmark(c: &mut Criterion)
{
    // data used for the push
    let seed = 42;
    let data = ParetoElement::sample_n(500000, seed);
    // fronts
    let cutpoint = data.len() / 8;
    let front: ParetoFront<_> = data[cutpoint..].iter().cloned().collect();
    let data: Vec<_> = data[..cutpoint].iter().cloned().collect();
    // actual bench
    c.bench_function("extend_500000", |b| b.iter(|| extend_fronts(front.clone(), data.clone())));
}

/// compares two implementation of the `push` function
/*fn comparison_benchmark(c: &mut Criterion)
{
    // data used for the push
    let seed = 42;
    let data = ParetoElement::sample_n(500000, seed);
    // fronts
    let cutpoint = data.len() / 2;
    let front: ParetoFront<_> = data[cutpoint..].iter().cloned().collect();
    let data: Vec<_> = data[..cutpoint].iter().cloned().collect();
    // compares various functions
    let mut group = c.benchmark_group("extend_500000");
    group.bench_function("extend", |b| b.iter(|| extend_fronts(front.clone(), data.clone())));
    group.bench_function("extend2", |b| b.iter(|| extend_fronts2(front.clone(), data.clone())));
    group.finish();
}*/

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
