use crate::{
    color::Color,
    point::Point,
    shape::{Circle, Shape},
};
use minifb::Key;
pub mod color;

pub mod point;
pub mod shape;
pub mod ui;
use ui::{UI, minifb::MinifbUI};

fn draw_shape_in_ui<S: Shape, I: UI>(shape: S, ui: &mut I) {
    let height = ui.height_in_pixels();
    let width = ui.width_in_pixels();
    for i in 0..height {
        for j in 0..width {
            let j_cm = ui.in_cm(j);
            let i_cm = ui.in_cm(i);
            if shape.does_satisfy(Point { x: j_cm, y: i_cm }) {
                ui.fill_with(j, i, Color::new(255, 255, 0))
            }
        }
    }

    ui.render();
}

fn main() {
    let c = Circle {
        radius: 5.0,
        centre: Point::new(5.0, 5.0),
    };

    let mut ui = MinifbUI::new(600, 600, 141.76);
    draw_shape_in_ui(c, &mut ui);

    while ui.window.is_open() && !ui.window.is_key_down(Key::Escape) {
        ui.render();
    }
}
