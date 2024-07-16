use std::collections::VecDeque;

use ggez::glam::Vec2;
use ggez::graphics::{Canvas, Color, DrawParam, Quad, Rect};

use crate::assets::Assets;
use crate::constants::*;

pub struct Effects {
    stars: Vec<Star>,
    trail: VecDeque<Vec2>,
}

struct Star {
    pos_frac: Vec2,
    speed: f32,
    size: f32,
}

impl Effects {
    pub fn new() -> Effects {
        let stars = (0..STAR_COUNT)
            .map(|_| Star {
                pos_frac: Vec2::new(fastrand::f32(), fastrand::f32()),
                speed: 6.0 + fastrand::f32() * 14.0,
                size: 1.0 + fastrand::f32() * 1.6,
            })
            .collect();
        Effects {
            stars,
            trail: VecDeque::with_capacity(TRAIL_LEN),
        }
    }

    pub fn track_ball(&mut self, pos: Vec2) {
        self.trail.push_back(pos);
        if self.trail.len() > TRAIL_LEN {
            self.trail.pop_front();
        }
    }

    pub fn clear_trail(&mut self) {
        self.trail.clear();
    }

    pub fn update(&mut self, dt: f32) {
        for s in &mut self.stars {
            s.pos_frac.y += s.speed * s.size * dt / SCREEN_H;
            if s.pos_frac.y > 1.0 {
                s.pos_frac.y -= 1.0;
                s.pos_frac.x = fastrand::f32();
            }
        }
    }

    pub fn draw_stars(&self, canvas: &mut Canvas, area: Rect) {
        for s in &self.stars {
            let alpha = 0.10 + 0.08 * s.size;
            canvas.draw(
                &Quad,
                DrawParam::new()
                    .dest_rect(Rect::new(
                        area.x + s.pos_frac.x * area.w,
                        area.y + s.pos_frac.y * area.h,
                        s.size,
                        s.size,
                    ))
                    .color(Color::new(1.0, 1.0, 1.0, alpha)),
            );
        }
    }

    pub fn draw_trail(&self, canvas: &mut Canvas, assets: &Assets) {
        let n = self.trail.len();
        for (i, pos) in self.trail.iter().enumerate() {
            let f = (i + 1) as f32 / n as f32;
            canvas.draw(
                &assets.ball,
                DrawParam::new()
                    .dest(*pos)
                    .scale(Vec2::splat(0.85 * f))
                    .color(Color {
                        a: 0.30 * f,
                        ..TRAIL_COLOR
                    }),
            );
        }
    }
}
