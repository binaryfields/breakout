use ggez::event::EventHandler;
use ggez::glam::Vec2;
use ggez::graphics::Canvas;
use ggez::input::keyboard::KeyCode;
use ggez::input::mouse::{self, MouseButton};
use ggez::{Context, GameResult};

use crate::assets::Assets;
use crate::ball::Ball;
use crate::brick::{self, Brick};
use crate::collision;
use crate::constants::*;
use crate::paddle::Paddle;
use crate::ui;

pub struct Game {
    paddle: Paddle,
    ball: Ball,
    bricks: Vec<Brick>,
    assets: Assets,
    score: u32,
    lives: u32,
    ball_speed: f32,
    phase: Phase,
}

impl Game {
    pub fn new(ctx: &mut Context) -> GameResult<Game> {
        let mut game = Game {
            paddle: Paddle::new(),
            ball: Ball::new(Vec2::ZERO),
            bricks: brick::build_grid(),
            assets: Assets::new(ctx)?,
            score: 0,
            lives: START_LIVES,
            ball_speed: BALL_SPEED_START_PPS,
            phase: Phase::Ready,
        };
        game.reset_ball();
        Ok(game)
    }

    fn reset(&mut self, ctx: &mut Context) -> GameResult {
        self.bricks = brick::build_grid();
        self.score = 0;
        self.lives = START_LIVES;
        self.ball_speed = BALL_SPEED_START_PPS;
        self.reset_ball();
        self.switch_phase(ctx, Phase::Ready)
    }

    fn reset_ball(&mut self) {
        self.ball.pos = Vec2::new(
            self.paddle.center_x(),
            self.paddle.rect.y - self.ball.radius - 2.0,
        );
        self.ball.vel = Vec2::ZERO;
    }

    fn launch_ball(&mut self) {
        let angle = (fastrand::f32() * 2.0 - 1.0) * LAUNCH_SPREAD_DEG.to_radians();
        self.ball.vel = Vec2::new(angle.sin(), -angle.cos()) * self.ball_speed;
    }

    fn lose_life(&mut self, ctx: &mut Context) -> GameResult {
        self.lives -= 1;
        if self.lives == 0 {
            self.switch_phase(ctx, Phase::GameOver)
        } else {
            self.reset_ball();
            self.switch_phase(ctx, Phase::Ready)
        }
    }

    fn switch_phase(&mut self, ctx: &mut Context, phase: Phase) -> GameResult {
        self.phase = phase;
        let playing = phase == Phase::Playing;
        mouse::set_cursor_hidden(ctx, playing);
        mouse::set_cursor_grabbed(ctx, playing)
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

    fn process_brick_collision(&mut self, ctx: &mut Context) -> GameResult {
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
            if self.bricks.iter().all(|b| !b.alive) {
                self.switch_phase(ctx, Phase::Win)?;
            }
        }
        Ok(())
    }
}

impl EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let dt = ctx.time.delta().as_secs_f32().min(1.0 / 30.0);
        self.control_paddle(ctx, dt);
        match self.phase {
            Phase::Ready => self.reset_ball(),
            Phase::Playing => {
                self.ball.update(dt);
                self.process_paddle_collision();
                self.process_brick_collision(ctx)?;
                if self.phase == Phase::Playing && self.ball.pos.y - self.ball.radius > SCREEN_H {
                    self.lose_life(ctx)?;
                }
            }
            Phase::GameOver | Phase::Win => {}
        }
        Ok(())
    }

    fn mouse_button_down_event(
        &mut self,
        ctx: &mut Context,
        button: MouseButton,
        _x: f32,
        _y: f32,
    ) -> GameResult {
        if button != MouseButton::Left {
            return Ok(());
        }
        match self.phase {
            Phase::Ready => {
                self.launch_ball();
                self.switch_phase(ctx, Phase::Playing)
            }
            Phase::GameOver | Phase::Win => self.reset(ctx),
            Phase::Playing => Ok(()),
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(ctx, BG_COLOR);
        for brick in self.bricks.iter().filter(|b| b.alive) {
            brick.draw(&mut canvas);
        }
        self.paddle.draw(&mut canvas);
        self.ball.draw(&mut canvas, &self.assets);
        ui::draw_hud(&mut canvas, &self.assets, self.score, self.lives);
        ui::draw_overlay(ctx, &mut canvas, self.phase, self.score)?;
        canvas.finish(ctx)
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum Phase {
    Ready,
    Playing,
    GameOver,
    Win,
}
