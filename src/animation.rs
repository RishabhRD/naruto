use crate::{
    point::Point,
    shape::{Circle, Shape},
};
use rand::Rng;

fn random_float(n: f64) -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0.0..n)
}

pub trait Animation {
    type AnimationShape: Shape;

    fn at(&self, ms: usize) -> Option<Self::AnimationShape>;
}

pub struct RandomCircleAnimation {}

impl Animation for RandomCircleAnimation {
    type AnimationShape = Circle;

    fn at(&self, _: usize) -> Option<Self::AnimationShape> {
        let r = random_float(5.0);
        Some(Circle {
            radius: r,
            centre: Point::new(5.0, 5.0),
        })
    }
}
