// benchmarking lib
use criterion::{criterion_group, criterion_main, Criterion};
// element type to do our benchmarks on
mod pareto_element;
use pareto_element::ParetoElementCircle as ParetoElement;
// pareto front
use pareto_front::ParetoFront;

/// merges two pareto fronts
fn merge_fronts(mut front1: ParetoFront<ParetoElement>,
                front2: ParetoFront<ParetoElement>)
                -> ParetoFront<ParetoElement>
{
    front1.merge(front2);
    front1
}

/*fn merge_fronts2(mut front1: ParetoFront<ParetoElement>,
                 front2: ParetoFront<ParetoElement>)
                 -> ParetoFront<ParetoElement>
{
    front1.merge2(front2);
    front1
}*/

/// measures the speed of several insertions in a row
fn criterion_benchmark(c: &mut Criterion)
{
    // data used for the push
    let seed = 42;
    let data = ParetoElement::sample_n(500000, seed);
    // fronts
    let cutpoint = data.len() / 8;
    let front1: ParetoFront<_> = data[..cutpoint].iter().cloned().collect();
    let front2: ParetoFront<_> = data[cutpoint..].iter().cloned().collect();
    // actual bench
    c.bench_function("merge_500000", |b| b.iter(|| merge_fronts(front1.clone(), front2.clone())));
}

// compares two implementation of the `push` function
/*fn comparison_benchmark(c: &mut Criterion)
{
    // data used for the push
    let seed = 42;
    let data = ParetoElement::sample_n(500000, seed);
    // fronts
    let cutpoint = data.len() / 2;
    let front1: ParetoFront<_> = data[..cutpoint].iter().cloned().collect();
    let front2: ParetoFront<_> = data[cutpoint..].iter().cloned().collect();
    // compares various functions
    let mut group = c.benchmark_group("merge_500000");
    group.bench_function("merge", |b| b.iter(|| merge_fronts(front1.clone(), front2.clone())));
    group.bench_function("merge2", |b| b.iter(|| merge_fronts2(front1.clone(), front2.clone())));
    group.finish();
}*/

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
