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

    pub fn draw(&self, canvas: &mut Canvas) {
        canvas.draw(
            &Quad,
            DrawParam::new().dest_rect(self.rect).color(PADDLE_COLOR),
        );
    }
}
