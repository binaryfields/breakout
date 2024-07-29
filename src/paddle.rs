use ggez::graphics::{Canvas, DrawParam, Quad, Rect};

use crate::constants::*;

pub struct Paddle {
    pub rect: Rect,
}

impl Paddle {
    pub fn new() -> Paddle {
        Paddle {
            rect: Rect::new(
                (SCREEN_W - PADDLE_W) / 2.0,
                SCREEN_H - PADDLE_Y_OFFSET - PADDLE_H,
                PADDLE_W,
                PADDLE_H,
            ),
        }
    }

    pub fn move_to(&mut self, x: f32) {
        self.rect.x = (x - self.rect.w / 2.0).clamp(0.0, SCREEN_W - self.rect.w);
    }

    pub fn move_by(&mut self, dx: f32) {
        self.rect.x = (self.rect.x + dx).clamp(0.0, SCREEN_W - self.rect.w);
    }

    pub fn draw(&self, canvas: &mut Canvas, squash: f32) {
        let w = self.rect.w * (1.0 + 0.25 * squash);
        let h = self.rect.h * (1.0 - 0.35 * squash);
        let rect = Rect::new(
            self.rect.x - (w - self.rect.w) / 2.0,
            self.rect.y + (self.rect.h - h) / 2.0,
            w,
            h,
        );
        canvas.draw(&Quad, DrawParam::new().dest_rect(rect).color(PADDLE_COLOR));
    }

    pub fn center_x(&self) -> f32 {
        self.rect.x + self.rect.w / 2.0
    }
}
