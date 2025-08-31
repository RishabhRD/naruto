use crate::point::Point;

pub trait Shape {
    fn does_satisfy(&self, point: Point) -> bool;
}

pub struct Circle {
    pub radius: f64,
    pub centre: Point,
}

fn float_equals(a: f64, b: f64) -> bool {
    (b - a).abs() <= 0.2
}

impl Shape for Circle {
    fn does_satisfy(&self, point: Point) -> bool {
        let x = point.x - self.centre.x;
        let y = point.y - self.centre.y;

        x * x + y * y <= self.radius * self.radius
    }
}
