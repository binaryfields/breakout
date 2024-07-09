use ggez::glam::Vec2;
use ggez::graphics::{Color, DrawMode, Mesh};
use ggez::{Context, GameResult};

use crate::constants::BALL_RADIUS;

pub struct Assets {
    pub ball: Mesh,
}

impl Assets {
    pub fn new(ctx: &mut Context) -> GameResult<Assets> {
        let ball = Mesh::new_circle(
            ctx,
            DrawMode::fill(),
            Vec2::ZERO,
            BALL_RADIUS,
            0.2,
            Color::WHITE,
        )?;
        Ok(Assets { ball })
    }
}
