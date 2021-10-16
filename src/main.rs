#![allow(dead_code)]
mod pareto_front;

//---------------------------------------------------------------------------------------
// TEST DATA

/// test element
#[derive(Debug)]
struct ParetoElement
{
    cost: usize,
    quality: f32,
    score: i64
}

impl pareto_front::Dominate for ParetoElement
{
    fn dominate(&self, x: &Self) -> bool
    {
        (self.cost <= x.cost) && (self.quality >= x.quality) && (self.score >= x.score)
    }
}

//---------------------------------------------------------------------------------------
// MAIN

fn main()
{
    // data to be put in the front
    let x = ParetoElement { cost: 35, quality: 0.5, score: 4 };
    let y = ParetoElement { cost: 350, quality: 0.05, score: 2 };
    let z = ParetoElement { cost: 5, quality: 0.25, score: 5 };

    // insertions in a front
    let mut front = pareto_front::ParetoFront::new();
    front.push(x);
    front.push(y);
    front.push(z);

    // display of the result
    for (i, element) in front.iter().enumerate()
    {
        println!("{}: {:?}", i, element);
    }
}
