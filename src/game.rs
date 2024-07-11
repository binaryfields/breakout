use ggez::event::EventHandler;
use ggez::glam::Vec2;
use ggez::graphics::Canvas;
use ggez::input::keyboard::KeyCode;
use ggez::{Context, GameResult};

use crate::assets::Assets;
use crate::ball::Ball;
use crate::brick::{self, Brick};
use crate::collision;
use crate::constants::*;
use crate::paddle::Paddle;

pub struct Game {
    paddle: Paddle,
    ball: Ball,
    bricks: Vec<Brick>,
    assets: Assets,
    score: u32,
    ball_speed: f32,
}

impl Game {
    pub fn new(ctx: &mut Context) -> GameResult<Game> {
        let mut ball = Ball::new(Vec2::new(SCREEN_W / 2.0, SCREEN_H * 0.6));
        ball.vel = Vec2::new(0.35, -1.0).normalize() * BALL_SPEED_START_PPS;
        Ok(Game {
            paddle: Paddle::new(),
            ball,
            bricks: brick::build_grid(),
            assets: Assets::new(ctx)?,
            score: 0,
            ball_speed: BALL_SPEED_START_PPS,
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

    fn process_paddle_collision(&mut self) {
        if self.ball.vel.y > 0.0 && self.ball.bounding_box().overlaps(&self.paddle.rect) {
            let half_w = self.paddle.rect.w / 2.0;
            let t = ((self.ball.pos.x - self.paddle.center_x()) / half_w).clamp(-1.0, 1.0);
            let angle = t * MAX_BOUNCE_ANGLE_DEG.to_radians();
            let speed = self.ball.vel.length();
            self.ball.vel = Vec2::new(angle.sin(), -angle.cos()) * speed;
            self.ball.pos.y = self.paddle.rect.y - self.ball.radius;
        }
    }

    fn process_brick_collision(&mut self, ctx: &mut Context) {
        let mut hit_points = None;
        for brick in &mut self.bricks {
            if brick.alive
                && collision::bounce_ball_off_rect(
                    &mut self.ball.pos,
                    &mut self.ball.vel,
                    self.ball.radius,
                    &brick.rect,
                )
            {
                brick.alive = false;
                hit_points = Some(brick.points);
                break;
            }
        }
        if let Some(points) = hit_points {
            self.score += points;
            self.ball_speed = (self.ball_speed + BALL_SPEED_INCREMENT_PPS).min(BALL_SPEED_MAX_PPS);
            self.ball.vel = self.ball.vel.normalize() * self.ball_speed;
            ctx.gfx
                .set_window_title(&format!("Breakout \u{2014} score {}", self.score));
        }
    }
}

impl EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let dt = ctx.time.delta().as_secs_f32().min(1.0 / 30.0);
        self.control_paddle(ctx, dt);
        self.ball.update(dt);
        self.process_paddle_collision();
        self.process_brick_collision(ctx);
        if self.ball.pos.y + self.ball.radius > SCREEN_H {
            self.ball.pos.y = SCREEN_H - self.ball.radius;
            self.ball.vel.y = -self.ball.vel.y.abs();
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(ctx, BG_COLOR);
        for brick in self.bricks.iter().filter(|b| b.alive) {
            brick.draw(&mut canvas);
        }
        self.paddle.draw(&mut canvas);
        self.ball.draw(&mut canvas, &self.assets);
        canvas.finish(ctx)
    }
}
