use ggez::glam::Vec2;
use ggez::graphics::{Canvas, Color, DrawParam, PxScale, Quad, Rect, Text};
use ggez::{Context, GameResult};

use crate::assets::Assets;
use crate::constants::*;
use crate::effects::Effects;
use crate::game::Phase;

struct Label<'a> {
    text: &'a str,
    y_frac: f32,
    font_px: f32,
    color: Color,
    scale: f32,
}

impl<'a> Label<'a> {
    fn new(text: &'a str, y_frac: f32, font_px: f32, color: Color) -> Self {
        Label {
            text,
            y_frac,
            font_px,
            color,
            scale: 1.0,
        }
    }

    fn draw(&self, ctx: &Context, canvas: &mut Canvas) -> GameResult {
        let mut text = Text::new(self.text);
        text.set_scale(PxScale::from(self.font_px));
        let dims = text.measure(ctx)?;
        let size = Vec2::new(dims.x * self.scale, dims.y * self.scale);
        let dest = Vec2::new(
            (SCREEN_W - size.x) / 2.0,
            SCREEN_H * self.y_frac - size.y / 2.0,
        );
        canvas.draw(
            &text,
            DrawParam::new()
                .dest(dest)
                .scale(Vec2::splat(self.scale))
                .color(self.color),
        );
        Ok(())
    }

    fn with_scale(mut self, scale: f32) -> Self {
        self.scale = scale;
        self
    }
}

pub fn draw_walls(canvas: &mut Canvas) {
    for x in [-WALL_THICKNESS, SCREEN_W] {
        let rect = Rect::new(x, PLAYFIELD_TOP, WALL_THICKNESS, SCREEN_H - PLAYFIELD_TOP);
        fill(canvas, rect, WALL_COLOR);
    }
}

pub fn draw_hud(canvas: &mut Canvas, assets: &Assets, score: u32, lives: u32) {
    let rule = Rect::new(0.0, PLAYFIELD_TOP - HUD_LINE_H, SCREEN_W, HUD_LINE_H);
    fill(canvas, rule, HUD_LINE_COLOR);

    let mut score_text = Text::new(format!("SCORE {score}"));
    score_text.set_scale(PxScale::from(HUD_TEXT_PX));
    canvas.draw(&score_text, DrawParam::new().dest(HUD_SCORE_POS));

    for i in 0..lives {
        canvas.draw(
            &assets.ball,
            DrawParam::new()
                .dest(Vec2::new(SCREEN_W - LIFE_SPACING * (i + 1) as f32, LIFE_Y))
                .scale(Vec2::splat(LIFE_SCALE))
                .color(LIVES_COLOR),
        );
    }
}

pub fn draw_overlay(
    ctx: &Context,
    canvas: &mut Canvas,
    phase: Phase,
    score: u32,
    effects: &Effects,
    area: Rect,
) -> GameResult {
    let (title, title_color) = match phase {
        Phase::Playing => return Ok(()),
        Phase::Ready => {
            return Label::new("CLICK TO LAUNCH", 0.62, 30.0, white(pulse(effects)))
                .draw(ctx, canvas)
        }
        Phase::Win => ("YOU WIN!", ROW_COLORS[2]),
        Phase::GameOver => ("GAME OVER", ROW_COLORS[0]),
    };

    fill(canvas, area, OVERLAY_DIM_COLOR);

    let fade = effects.transition();
    let score_text = format!("FINAL SCORE {score}");

    let lines = [
        Label::new(title, 0.38, 76.0, title_color).with_scale(ease_out_back(fade)),
        Label::new(&score_text, 0.50, 34.0, white(fade)),
        Label::new("CLICK TO RESTART", 0.60, 26.0, white(pulse(effects) * fade)),
    ];
    lines.iter().try_for_each(|line| line.draw(ctx, canvas))
}

fn white(alpha: f32) -> Color {
    Color::new(1.0, 1.0, 1.0, alpha)
}

fn pulse(effects: &Effects) -> f32 {
    PROMPT_PULSE_BASE + PROMPT_PULSE_AMP * (effects.elapsed() * PROMPT_PULSE_HZ).sin()
}

fn ease_out_back(t: f32) -> f32 {
    const C1: f32 = 1.70158;
    const C3: f32 = C1 + 1.0;
    let u = t - 1.0;
    1.0 + C3 * u.powi(3) + C1 * u.powi(2)
}

fn fill(canvas: &mut Canvas, rect: Rect, color: Color) {
    canvas.draw(&Quad, DrawParam::new().dest_rect(rect).color(color));
}
