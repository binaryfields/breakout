use ggez::event::EventHandler;
use ggez::glam::Vec2;
use ggez::graphics::{Canvas, Color, Rect};
use ggez::input::keyboard::KeyCode;
use ggez::input::mouse::{self, MouseButton};
use ggez::{Context, GameResult};

use crate::assets::Assets;
use crate::ball::Ball;
use crate::brick::{self, Brick};
use crate::collision;
use crate::constants::*;
use crate::effects::Effects;
use crate::paddle::Paddle;
use crate::ui;
use crate::viewport::Viewport;

pub struct Game {
    paddle: Paddle,
    ball: Ball,
    round: Round,
    assets: Assets,
    effects: Effects,
    phase: Phase,
}

struct Round {
    bricks: Vec<Brick>,
    score: u32,
    lives: u32,
    ball_speed: f32,
}

impl Default for Round {
    fn default() -> Self {
        Round {
            bricks: brick::build_grid(),
            score: 0,
            lives: START_LIVES,
            ball_speed: BALL_SPEED_START_PPS,
        }
    }
}

struct BrickHit {
    points: u32,
    rect: Rect,
    color: Color,
}

impl Game {
    pub fn new(ctx: &mut Context) -> GameResult<Game> {
        let mut game = Game {
            paddle: Paddle::new(),
            ball: Ball::new(Vec2::ZERO),
            round: Round::default(),
            assets: Assets::new(ctx)?,
            effects: Effects::new(),
            phase: Phase::Ready,
        };
        game.rest_ball_on_paddle();
        Ok(game)
    }

    fn reset(&mut self, ctx: &mut Context) -> GameResult {
        self.round = Round::default();
        self.effects.clear_trail();
        self.rest_ball_on_paddle();
        self.switch_phase(ctx, Phase::Ready)
    }

    fn rest_ball_on_paddle(&mut self) {
        self.ball.pos = Vec2::new(
            self.paddle.center_x(),
            self.paddle.rect.y - self.ball.radius - 2.0,
        );
        self.ball.vel = Vec2::ZERO;
    }

    fn launch_ball(&mut self) {
        let spread = LAUNCH_SPREAD_DEG.to_radians();
        let angle = fastrand::f32().mul_add(2.0, -1.0) * spread;
        self.ball.vel = upward_velocity(angle, self.round.ball_speed);
    }

    fn lose_life(&mut self, ctx: &mut Context) -> GameResult {
        self.round.lives -= 1;
        self.effects.clear_trail();
        if self.round.lives == 0 {
            return self.switch_phase(ctx, Phase::GameOver);
        }
        self.rest_ball_on_paddle();
        self.switch_phase(ctx, Phase::Ready)
    }

    fn switch_phase(&mut self, ctx: &mut Context, phase: Phase) -> GameResult {
        self.phase = phase;
        self.effects.start_transition();
        let playing = phase == Phase::Playing;
        mouse::set_cursor_hidden(ctx, playing);
        mouse::set_cursor_grabbed(ctx, playing)
    }

    fn control_paddle(&mut self, ctx: &Context, dt: f32) {
        let d = ctx.mouse.delta();
        if d.x != 0.0 || d.y != 0.0 {
            let x = viewport(ctx).to_logical_x(ctx.mouse.position().x);
            self.paddle.move_to(x);
            return;
        }
        let held = |keys: &[KeyCode]| keys.iter().any(|&k| ctx.keyboard.is_key_pressed(k));
        let dir = f32::from(held(&[KeyCode::Right, KeyCode::D]))
            - f32::from(held(&[KeyCode::Left, KeyCode::A]));
        if dir != 0.0 {
            self.paddle.move_by(dir * PADDLE_SPEED_PPS * dt);
        }
    }

    fn update_playing(&mut self, ctx: &mut Context, dt: f32) -> GameResult {
        self.ball.update(dt);
        self.bounce_off_paddle();
        if let Some(hit) = self.bounce_off_bricks() {
            self.apply_hit(ctx, hit)?;
        }
        self.effects.track_ball(self.ball.pos);

        if self.phase == Phase::Playing && self.is_ball_lost() {
            self.lose_life(ctx)?;
        }
        Ok(())
    }

    fn bounce_off_paddle(&mut self) {
        let descending = self.ball.vel.y > 0.0;
        if !descending || !self.ball.bounding_box().overlaps(&self.paddle.rect) {
            return;
        }
        let offset = (self.ball.pos.x - self.paddle.center_x()) / (self.paddle.rect.w / 2.0);
        let angle = offset.clamp(-1.0, 1.0) * MAX_BOUNCE_ANGLE_DEG.to_radians();
        self.ball.vel = upward_velocity(angle, self.ball.vel.length());
        self.ball.pos.y = self.paddle.rect.y - self.ball.radius;
        self.effects.squash_paddle();
    }

    fn bounce_off_bricks(&mut self) -> Option<BrickHit> {
        let ball = &mut self.ball;
        let brick = self.round.bricks.iter_mut().filter(|b| b.alive).find(|b| {
            collision::bounce_ball_off_rect(&mut ball.pos, &mut ball.vel, ball.radius, &b.rect)
        })?;
        brick.alive = false;
        Some(BrickHit {
            points: brick.points,
            rect: brick.rect,
            color: brick.color,
        })
    }

    fn apply_hit(&mut self, ctx: &mut Context, hit: BrickHit) -> GameResult {
        self.round.score += hit.points;
        self.effects.spawn_burst(&hit.rect, hit.color);
        self.effects.spawn_popup(
            Vec2::new(hit.rect.x + hit.rect.w / 2.0, hit.rect.y),
            hit.points,
        );
        self.round.ball_speed =
            (self.round.ball_speed + BALL_SPEED_INCREMENT_PPS).min(BALL_SPEED_MAX_PPS);
        self.ball.vel = self.ball.vel.normalize() * self.round.ball_speed;

        if self.round.bricks.iter().all(|b| !b.alive) {
            self.switch_phase(ctx, Phase::Win)?;
        }
        Ok(())
    }

    fn is_ball_lost(&self) -> bool {
        self.ball.pos.y - self.ball.radius > SCREEN_H
    }
}

impl EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let dt = ctx.time.delta().as_secs_f32().min(MAX_FRAME_DT);
        self.effects.update(dt);
        self.control_paddle(ctx, dt);
        match self.phase {
            Phase::Ready => self.rest_ball_on_paddle(),
            Phase::Playing => self.update_playing(ctx, dt)?,
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
        match (button, self.phase) {
            (MouseButton::Left, Phase::Ready) => {
                self.launch_ball();
                self.switch_phase(ctx, Phase::Playing)
            }
            (MouseButton::Left, Phase::GameOver | Phase::Win) => self.reset(ctx),
            _ => Ok(()),
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let vp = viewport(ctx);
        let mut canvas = Canvas::from_frame(ctx, BG_COLOR);
        canvas.set_screen_coordinates(vp.rect);

        self.effects.draw_stars(&mut canvas, vp.rect);
        ui::draw_walls(&mut canvas);
        for brick in self.round.bricks.iter().filter(|b| b.alive) {
            brick.draw(&mut canvas);
        }
        self.effects.draw_trail(&mut canvas, &self.assets);
        self.ball.draw(&mut canvas, &self.assets);
        self.paddle.draw(&mut canvas, self.effects.paddle_squash());
        self.effects.draw_particles(&mut canvas);
        self.effects.draw_popups(&mut canvas);
        ui::draw_hud(
            &mut canvas,
            &self.assets,
            self.round.score,
            self.round.lives,
        );
        ui::draw_overlay(
            ctx,
            &mut canvas,
            self.phase,
            self.round.score,
            &self.effects,
            vp.rect,
        )?;
        canvas.finish(ctx)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Phase {
    Ready,
    Playing,
    GameOver,
    Win,
}

fn upward_velocity(angle: f32, speed: f32) -> Vec2 {
    Vec2::new(angle.sin(), -angle.cos()) * speed
}

fn viewport(ctx: &Context) -> Viewport {
    let (w, h) = ctx.gfx.drawable_size();
    Viewport::new(w, h)
}
