use minifb::{Window, WindowOptions};

use crate::{UI, color::Color};

pub struct MinifbUI {
    pub window: Window,
    buffer: Vec<u32>,
    width: usize,
    height: usize,
    dpi: f64,
}

impl MinifbUI {
    pub fn new(width: usize, height: usize, dpi: f64) -> Self {
        let window = Window::new("Naruto", width, height, WindowOptions::default()).unwrap();

        MinifbUI {
            window,
            buffer: vec![0; width * height],
            width,
            height,
            dpi,
        }
    }
}

impl UI for MinifbUI {
    fn in_cm(&self, pixels: usize) -> f64 {
        (pixels as f64) * 2.54 / self.dpi
    }

    fn width_in_pixels(&self) -> usize {
        self.width
    }

    fn height_in_pixels(&self) -> usize {
        self.height
    }

    fn fill_with(&mut self, x: usize, y: usize, color: Color) {
        if x < self.width && y < self.height {
            self.buffer[y * self.width + x] =
                (color.r as u32) << 16 | (color.g as u32) << 8 | (color.b as u32);
        }
    }

    fn render(&mut self) {
        self.window
            .update_with_buffer(&self.buffer, self.width, self.height)
            .unwrap();
    }

    fn clear(&mut self) {
        for pixel in self.buffer.iter_mut() {
            *pixel = 0x000000;
        }
    }
}
