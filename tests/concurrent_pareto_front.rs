mod pareto_element;
use pareto_element::ParetoElement;
use pareto_front::{ConcurrentParetoFront, ParetoFront};
use rayon::prelude::*;

/// adds 1000 elements to a ParetoFront and a ConcurrentParetoFront
/// check the result to ensure they are the same
#[test]
fn insert_concurrent()
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

    // concurrent front
    let conc_front = ConcurrentParetoFront::new();
    data.par_iter().for_each(|x| {
                       conc_front.push(*x);
                   });
    let mut conc_front: Vec<_> = conc_front.into();
    conc_front.sort();

    // checks for equality (after a sort to remove ordering from the potential sources of difference)
    assert_eq!(seq_front.len(), conc_front.len());
    for (idx, (s, c)) in seq_front.iter().zip(conc_front.iter()).enumerate()
    {
        if s != c
        {
            println!("{}: {:?} != {:?}", idx, s, c)
        }
    }
    assert!(seq_front.eq(&conc_front))
}
