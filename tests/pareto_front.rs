mod pareto_element;
use pareto_element::ParetoElement;
use pareto_front::ParetoFront;

/// adds 3 elements to a pareto front and checks to see if the result is correct
#[test]
fn push3()
{
    // data to be put in the front
    let x = ParetoElement { cost: 35, quality: 50, score: 4 };
    let y = ParetoElement { cost: 350, quality: 5, score: 2 };
    let z = ParetoElement { cost: 6, quality: 25, score: 5 };
    let v = ParetoElement { cost: 5, quality: 25, score: 5 };

    // insertions in a new front
    let mut front = ParetoFront::new();
    let x_is_optimal = front.push(x);
    assert!(x_is_optimal);
    let y_is_optimal = front.push(y);
    assert!(!y_is_optimal); // this element shouldn't be on the front
    let z_is_optimal = front.push(z);
    assert!(z_is_optimal); // this element should be on the front for now
    let v_is_optimal = front.push(v);
    assert!(v_is_optimal);

    // converts the front into a vector for ease of analysis
    let front_vec: Vec<ParetoElement> = front.into();

    // checks size and content of the front
    assert_eq!(front_vec.len(), 2);
    assert!(front_vec.contains(&x));
    assert!(front_vec.contains(&v));
}

/// test the associativity of the push operation
#[test]
fn push_associativity()
{
    // data to be put in the front
    let seed = 42;
    let mut data = ParetoElement::sample_n(1000, seed);

    // sequential front
    let mut seq_front = ParetoFront::new();
    data.iter().for_each(|x| {
                   seq_front.push(*x);
               });
    let mut seq_front: Vec<_> = seq_front.into();
    seq_front.sort();

    // sequential front with different insertion order
    data.sort();
    let mut sort_front = ParetoFront::new();
    data.iter().for_each(|x| {
                   sort_front.push(*x);
               });
    let mut sort_front: Vec<_> = sort_front.into();
    sort_front.sort();

    // check for equality between both front
    assert_eq!(seq_front.len(), sort_front.len());
    assert!(seq_front.eq(&sort_front));
}
