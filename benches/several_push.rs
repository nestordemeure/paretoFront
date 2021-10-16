use criterion::{criterion_group, criterion_main, Criterion};
mod test_data;
use test_data::ParetoElement;
use pareto_front::ParetoFront;

/// generates n data points and insert them one after the other in a pareto front
fn generate_front(data: &[ParetoElement]) -> ParetoFront<ParetoElement>
{
    let mut front = ParetoFront::new();
    for x in data
    {
        front.push(*x);
    }
    front
}

fn criterion_benchmark(c: &mut Criterion)
{
    // data used for the bench
    let seed = 42;
    let data = ParetoElement::sample_n(1000, seed);
    // actual bench
    c.bench_function("insert 1000", |b| b.iter(|| generate_front(&data)));
}

/*fn comparison_benchmark(c: &mut Criterion)
{
    // data used for the bench
    let seed = 42;
    let data = ParetoElement::sample_n(1000, seed);
    // compares various functions
    let mut group = c.benchmark_group("compare_push_1000");
    group.bench_function("push", |b| b.iter(|| generate_front(&data)));
    group.bench_function("push2", |b| b.iter(|| generate_front2(&data)));
    group.finish();
}*/

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
