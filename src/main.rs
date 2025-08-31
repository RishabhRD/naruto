use crate::{
    animation::{Animation, RandomCircleAnimation},
    color::Color,
    point::Point,
    shape::{Circle, Shape},
};
use minifb::Key;
use std::thread;
use std::time::Duration;
pub mod color;

pub mod animation;
pub mod point;
pub mod shape;
pub mod ui;
use ui::{UI, minifb::MinifbUI};

fn draw_shape_in_ui<S: Shape, I: UI>(shape: S, ui: &mut I) {
    let height = ui.height_in_pixels();
    let width = ui.width_in_pixels();
    let c = Color::random();
    for i in 0..height {
        for j in 0..width {
            let j_cm = ui.in_cm(j);
            let i_cm = ui.in_cm(i);
            if shape.does_satisfy(Point { x: j_cm, y: i_cm }) {
                ui.fill_with(j, i, c.clone())
            }
        }
    }
}

fn draw_animation_until<A, U, F>(animation: &A, ui: &mut U, mut until: F)
where
    A: Animation,
    U: UI,
    F: FnMut(&mut U) -> bool,
{
    let mut cur = 0;
    let interval = 100;
    while until(ui) {
        let s = animation.at(cur);
        if s.is_none() {
            break;
        }
        draw_shape_in_ui(s.unwrap(), ui);
        cur += interval;
        ui.render();
        thread::sleep(Duration::from_millis(interval as u64));
        ui.clear()
    }
}

fn main() {
    let c = Circle {
        radius: 5.0,
        centre: Point::new(5.0, 5.0),
    };

    let mut ui = MinifbUI::new(600, 600, 141.76);
    draw_shape_in_ui(c, &mut ui);

    let animation = RandomCircleAnimation {};
    draw_animation_until(&animation, &mut ui, |ui| {
        !ui.window.is_key_down(Key::Escape)
    });
}
