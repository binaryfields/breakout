use ggez::event::EventHandler;
use ggez::glam::Vec2;
use ggez::graphics::Canvas;
use ggez::input::keyboard::KeyCode;
use ggez::{Context, GameResult};

use crate::assets::Assets;
use crate::ball::Ball;
use crate::constants::*;
use crate::paddle::Paddle;

pub struct Game {
    paddle: Paddle,
    ball: Ball,
    assets: Assets,
}

impl Game {
    pub fn new(ctx: &mut Context) -> GameResult<Game> {
        let mut ball = Ball::new(Vec2::new(SCREEN_W / 2.0, SCREEN_H * 0.6));
        ball.vel = Vec2::new(0.35, -1.0).normalize() * BALL_SPEED_START_PPS;
        Ok(Game {
            paddle: Paddle::new(),
            ball,
            assets: Assets::new(ctx)?,
        })
    }

    fn control_paddle(&mut self, ctx: &Context, dt: f32) {
        let mouse_delta = ctx.mouse.delta();
        if mouse_delta.x != 0.0 || mouse_delta.y != 0.0 {
            self.paddle.move_to(ctx.mouse.position().x);
            return;
        }
        let held = |code: KeyCode| ctx.keyboard.is_key_pressed(code);
        let mut dir = 0.0;
        if held(KeyCode::Left) || held(KeyCode::A) {
            dir -= 1.0;
        }
        if held(KeyCode::Right) || held(KeyCode::D) {
            dir += 1.0;
        }
        if dir != 0.0 {
            self.paddle.move_by(dir * PADDLE_SPEED_PPS * dt);
        }
    }
}

impl EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let dt = ctx.time.delta().as_secs_f32().min(1.0 / 30.0);
        self.control_paddle(ctx, dt);
        self.ball.update(dt);
        if self.ball.pos.y + self.ball.radius > SCREEN_H {
            self.ball.pos.y = SCREEN_H - self.ball.radius;
            self.ball.vel.y = -self.ball.vel.y.abs();
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(ctx, BG_COLOR);
        self.paddle.draw(&mut canvas);
        self.ball.draw(&mut canvas, &self.assets);
        canvas.finish(ctx)
    }
}
