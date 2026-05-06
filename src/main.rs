#![allow(dead_code)]
use pareto_front::{Dominate, ParetoFront};

/// test element
#[derive(Debug, PartialEq)]
struct ParetoElement {
    partial_data: i32,
    rest_of_the_data: Option<i32>,
}

/// implement the `Dominate` trait
impl Dominate for ParetoElement {
    fn dominate(&self, x: &Self) -> bool {
        self.partial_data < x.partial_data
    }
}

fn main() {
    // data to be put in the front
    let x = ParetoElement { partial_data: 1, rest_of_the_data: None };
    let y = ParetoElement { partial_data: 2, rest_of_the_data: None };
    let z = ParetoElement { partial_data: 3, rest_of_the_data: None };

    // insertions in the front
    let mut front = ParetoFront::new();

    fn compute_rest_expensively(pareto_element: &mut ParetoElement) -> bool {
        // Pretend this is super expensive
        pareto_element.rest_of_the_data =
            if pareto_element.partial_data % 2 == 0 { Some(pareto_element.partial_data * 2) } else { None };
        pareto_element.rest_of_the_data.is_some()
    }

    front.push_and_check(x, compute_rest_expensively);
    front.push_and_check(y, compute_rest_expensively);
    front.push_and_check(z, compute_rest_expensively);

    // display of the result
    for (i, element) in front.iter().enumerate() {
        println!("{}: {:?}", i, element);
    }
}
