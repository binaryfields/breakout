use ggez::glam::Vec2;
use ggez::graphics::{Canvas, DrawParam};

use crate::assets::Assets;
use crate::constants::*;

pub struct Ball {
    pub pos: Vec2,
    pub vel: Vec2,
    pub radius: f32,
}

impl Ball {
    pub fn new(pos: Vec2) -> Ball {
        Ball {
            pos,
            vel: Vec2::ZERO,
            radius: BALL_RADIUS,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.pos += self.vel * dt;
        self.bounce_off_walls();
    }

    pub fn draw(&self, canvas: &mut Canvas, assets: &Assets) {
        canvas.draw(&assets.ball, DrawParam::new().dest(self.pos));
    }

    fn bounce_off_walls(&mut self) {
        let r = self.radius;
        if self.pos.x - r < 0.0 {
            self.pos.x = r;
            self.vel.x = self.vel.x.abs();
        }
        if self.pos.x + r > SCREEN_W {
            self.pos.x = SCREEN_W - r;
            self.vel.x = -self.vel.x.abs();
        }
        if self.pos.y - r < PLAYFIELD_TOP {
            self.pos.y = PLAYFIELD_TOP + r;
            self.vel.y = self.vel.y.abs();
        }
    }
}
