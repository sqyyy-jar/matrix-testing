use crate::{PIXEL_HEIGHT, PIXEL_WIDTH, PREC_BITS, SPEED};

#[derive(Clone, Debug, Default)]
pub struct Vec2 {
    pub x: i32,
    pub y: i32,
}

impl Vec2 {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn convert(&self) -> Self {
        Self {
            x: (self.x >> PREC_BITS) * PIXEL_WIDTH,
            y: (self.y >> PREC_BITS) * PIXEL_HEIGHT,
        }
    }

    pub fn add(&self, other: &Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    pub fn mul(&self, amount: i32) -> Self {
        Self {
            x: self.x * amount,
            y: self.y * amount,
        }
    }

    /// Resize vector to [SPEED]
    pub fn resize(&self) -> Self {
        let len = ((self.x * self.x + self.y * self.y) as f64).sqrt() as i32;
        if len == 0 {
            return self.clone();
        }
        let x = self.x * SPEED / len;
        let y = self.y * SPEED / len;
        Self { x, y }
    }
}
