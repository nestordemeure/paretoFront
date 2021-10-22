use pareto_front::{ParetoFront, Dominate};

/// type of the elemnts to be inserted in the front
#[derive(Debug, Clone, Copy, PartialEq)]
struct ParetoElement
{
    cost: usize,
    quality: f32,
    score: i64
}

/// implement the `Dominate` trait
impl Dominate for ParetoElement
{
    fn dominate(&self, x: &Self) -> bool
    {
        (self.cost <= x.cost) && (self.quality >= x.quality) && (self.score >= x.score) && (self != x)
    }
}

/// adds 3 elements to a pareto front and checks to see if the result is correct
#[test]
fn insert3()
{
    // data to be put in the front
    let x = ParetoElement { cost: 35, quality: 0.5, score: 4 };
    let y = ParetoElement { cost: 350, quality: 0.05, score: 2 };
    let z = ParetoElement { cost: 6, quality: 0.25, score: 5 };
    let v = ParetoElement { cost: 5, quality: 0.25, score: 5 };

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
