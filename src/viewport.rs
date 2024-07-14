use ggez::graphics::Rect;

use crate::constants::*;

pub struct Viewport {
    pub scale: f32,
    pub rect: Rect,
}

impl Viewport {
    pub fn new(window_w: f32, window_h: f32) -> Viewport {
        let scale = (window_w / SCREEN_W).min(window_h / SCREEN_H);
        let scale = if scale.is_finite() && scale > 0.0 {
            scale
        } else {
            1.0
        };
        let logical_w = window_w / scale;
        let logical_h = window_h / scale;
        Viewport {
            scale,
            rect: Rect::new(
                -(logical_w - SCREEN_W) / 2.0,
                -(logical_h - SCREEN_H) / 2.0,
                logical_w,
                logical_h,
            ),
        }
    }

    pub fn to_logical_x(&self, window_x: f32) -> f32 {
        window_x / self.scale + self.rect.x
    }
}
