use std::collections::VecDeque;
use std::f32::consts::TAU;

use ggez::glam::Vec2;
use ggez::graphics::{Canvas, Color, DrawParam, PxScale, Quad, Rect, Text};

use crate::assets::Assets;
use crate::constants::*;

struct Star {
    pos_frac: Vec2,
    speed: f32,
    size: f32,
}

struct Particle {
    pos: Vec2,
    vel: Vec2,
    life: f32,
    max_life: f32,
    size: f32,
    color: Color,
}

struct Popup {
    pos: Vec2,
    text: String,
    life: f32,
}

pub struct Effects {
    stars: Vec<Star>,
    trail: VecDeque<Vec2>,
    elapsed: f32,
    transition: f32,
    particles: Vec<Particle>,
    popups: Vec<Popup>,
    paddle_squash: f32,
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
            elapsed: 0.0,
            transition: 1.0,
            particles: Vec::new(),
            popups: Vec::new(),
            paddle_squash: 0.0,
        }
    }

    pub fn paddle_squash(&self) -> f32 {
        self.paddle_squash
    }

    pub fn elapsed(&self) -> f32 {
        self.elapsed
    }

    pub fn transition(&self) -> f32 {
        self.transition
    }

    pub fn spawn_burst(&mut self, rect: &Rect, color: Color) {
        for _ in 0..PARTICLES_PER_BRICK {
            let pos = Vec2::new(
                rect.x + fastrand::f32() * rect.w,
                rect.y + fastrand::f32() * rect.h,
            );
            let angle = fastrand::f32() * TAU;
            let speed = 60.0 + fastrand::f32() * 240.0;
            let max_life = 0.35 + fastrand::f32() * 0.45;
            self.particles.push(Particle {
                pos,
                vel: Vec2::new(angle.cos() * speed, angle.sin() * speed - 60.0),
                life: max_life,
                max_life,
                size: 3.0 + fastrand::f32() * 4.0,
                color,
            });
        }
    }

    pub fn spawn_popup(&mut self, pos: Vec2, points: u32) {
        self.popups.push(Popup {
            pos,
            text: format!("+{points}"),
            life: POPUP_LIFE_SEC,
        });
    }

    pub fn squash_paddle(&mut self) {
        self.paddle_squash = 1.0;
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

    pub fn start_transition(&mut self) {
        self.transition = 0.0;
    }

    pub fn update(&mut self, dt: f32) {
        self.elapsed += dt;
        self.transition = (self.transition + dt / TRANSITION_TIME_SEC).min(1.0);
        self.paddle_squash = (self.paddle_squash - PADDLE_SQUASH_DECAY * dt).max(0.0);
        for s in &mut self.stars {
            s.pos_frac.y += s.speed * s.size * dt / SCREEN_H;
            if s.pos_frac.y > 1.0 {
                s.pos_frac.y -= 1.0;
                s.pos_frac.x = fastrand::f32();
            }
        }
        for p in &mut self.particles {
            p.vel.y += PARTICLE_GRAVITY * dt;
            p.pos += p.vel * dt;
            p.life -= dt;
        }
        self.particles.retain(|p| p.life > 0.0);
        for p in &mut self.popups {
            p.pos.y -= POPUP_RISE_PPS * dt;
            p.life -= dt;
        }
        self.popups.retain(|p| p.life > 0.0);
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

    pub fn draw_particles(&self, canvas: &mut Canvas) {
        for p in &self.particles {
            let f = p.life / p.max_life;
            let size = p.size * f.max(0.25);
            let mut color = p.color;
            color.a = f;
            canvas.draw(
                &Quad,
                DrawParam::new()
                    .dest_rect(Rect::new(
                        p.pos.x - size / 2.0,
                        p.pos.y - size / 2.0,
                        size,
                        size,
                    ))
                    .color(color),
            );
        }
    }

    pub fn draw_popups(&self, canvas: &mut Canvas) {
        for p in &self.popups {
            let f = p.life / POPUP_LIFE_SEC;
            let mut text = Text::new(p.text.as_str());
            text.set_scale(PxScale::from(22.0));
            canvas.draw(
                &text,
                DrawParam::new()
                    .dest(p.pos - Vec2::new(14.0, 0.0))
                    .color(Color::new(1.0, 1.0, 1.0, f)),
            );
        }
    }
}
