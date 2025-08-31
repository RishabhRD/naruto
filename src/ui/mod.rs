use crate::color::Color;

pub mod minifb;

pub trait UI {
    fn in_cm(&self, pixels: usize) -> f64;

    fn width_in_pixels(&self) -> usize;

    fn height_in_pixels(&self) -> usize;

    fn fill_with(&mut self, x: usize, y: usize, color: Color);

    fn render(&mut self);

    fn clear(&mut self);
}
