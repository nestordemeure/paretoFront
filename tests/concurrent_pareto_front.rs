mod pareto_element;
use pareto_element::ParetoElement;
use pareto_front::ParetoFront;

/// adds 1000 elements to a ParetoFront and a simulated ConcurrentParetoFront
/// check the result to ensure they are the same
/// in practice this tests the push and merge operation, reducing the list of potential culprits in case of bug
#[test]
fn simulated_push_concurrent()
{
    // data to be put in the front
    let seed = 42;
    let data = ParetoElement::sample_n(1000, seed);

    // sequential front
    let mut seq_front = ParetoFront::new();
    data.iter().for_each(|x| {
                   seq_front.push(*x);
               });
    let mut seq_front: Vec<_> = seq_front.into();
    seq_front.sort();

    // simulated concurrent front
    let mut sim_front: Vec<_> = (0..8).map(|_| ParetoFront::new()).collect();
    data.iter().enumerate().for_each(|(idx, x)| {
                               sim_front[idx % 8].push(*x);
                           });
    let sim_front = sim_front.into_iter()
                             .reduce(|mut front_acc, front| {
                                 front_acc.merge(front);
                                 front_acc
                             })
                             .unwrap_or_default();
    let mut sim_front: Vec<_> = sim_front.into();
    sim_front.sort();

    // check for equality with simulated front
    assert_eq!(seq_front.len(), sim_front.len());
    assert!(seq_front.eq(&sim_front));
}

/// adds 1000 elements to a ParetoFront and a ConcurrentParetoFront
/// check the result to ensure they are the same
#[test]
#[cfg(feature = "pareto_front_concurrent")]
fn push_concurrent()
{
    use pareto_front::ConcurrentParetoFront;
    use rayon::prelude::*;

    // data to be put in the front
    let seed = 42;
    let data = ParetoElement::sample_n(1000, seed);

    // sequential front
    let mut seq_front = ParetoFront::new();
    data.iter().for_each(|x| {
                   seq_front.push(*x);
               });
    let mut seq_front: Vec<_> = seq_front.into();
    seq_front.sort();

    // concurrent front
    let conc_front = ConcurrentParetoFront::new();
    data.par_iter().for_each(|x| {
                       conc_front.push(*x);
                   });
    let mut conc_front: Vec<_> = conc_front.into();
    conc_front.sort();

    // checks for equality with concurrent front
    assert_eq!(seq_front.len(), conc_front.len());
    assert!(seq_front.eq(&conc_front));
}
