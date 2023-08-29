use rand::Rng;
use vec::Vec2;

pub mod vec;

/// Precision bits
pub const PREC_BITS: i32 = 5;
/// Physical width
pub const PWIDTH: i32 = 128;
/// Virtual width
pub const VWIDTH: i32 = PWIDTH << PREC_BITS;

pub const SCREEN_WIDTH: i32 = 512;
pub const SCREEN_HEIGHT: i32 = 512;

pub const PIXEL_WIDTH: i32 = SCREEN_WIDTH / PWIDTH;
pub const PIXEL_HEIGHT: i32 = SCREEN_HEIGHT / PWIDTH;

pub const USED_SCREEN_WIDTH: i32 = PIXEL_WIDTH * PWIDTH;
pub const USED_SCREEN_HEIGHT: i32 = PIXEL_HEIGHT * PWIDTH;

pub const SPEED: i32 = 1 << PREC_BITS;

#[derive(Default)]
pub struct Dot {
    pub pos: Vec2,
    pub vel: Vec2,
}

impl Dot {
    pub fn draw(&self, frame: &mut [u8]) {
        let Vec2 { x, y } = self.pos.convert();
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let px = (i % SCREEN_WIDTH as usize) as i32;
            let py = (i / SCREEN_WIDTH as usize) as i32;
            if px > USED_SCREEN_WIDTH || py > USED_SCREEN_HEIGHT {
                continue;
            }
            let inside_pixel = px >= x && px < x + PIXEL_WIDTH && py >= y && py < y + PIXEL_HEIGHT;
            let rgba = if inside_pixel { [0xff; 4] } else { [0x80; 4] };
            pixel.copy_from_slice(&rgba);
        }
    }

    pub fn update(&mut self) {
        let mut dest = self.pos.add(&self.vel);
        while self.bounce(&mut dest) {}
        self.pos = dest;
    }

    pub fn randomize(&mut self) {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(-512..=512);
        let y = rng.gen_range(-512..=512);
        self.vel = Vec2::new(x, y).resize();
    }

    fn bounce(&mut self, dest: &mut Vec2) -> bool {
        let mut bounced = false;
        if dest.x < 0 {
            bounced = true;
            dest.x = -dest.x;
            self.vel.x = -self.vel.x;
        } else if dest.x >= VWIDTH {
            bounced = true;
            dest.x = VWIDTH - (dest.x - VWIDTH) - 1;
            self.vel.x = -self.vel.x;
        }
        if dest.y < 0 {
            bounced = true;
            dest.y = -dest.y;
            self.vel.y = -self.vel.y;
        } else if dest.y >= VWIDTH {
            bounced = true;
            dest.y = VWIDTH - (dest.y - VWIDTH) - 1;
            self.vel.y = -self.vel.y;
        }
        bounced
    }
}
