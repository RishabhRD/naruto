use rand::RngCore;

#[derive(Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b }
    }

    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        Color::new(
            (rng.next_u32() % 255) as u8,
            (rng.next_u32() % 255) as u8,
            (rng.next_u32() % 255) as u8,
        )
    }
}
