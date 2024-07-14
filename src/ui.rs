use ggez::glam::Vec2;
use ggez::graphics::{Canvas, Color, DrawParam, PxScale, Quad, Rect, Text};
use ggez::{Context, GameResult};

use crate::assets::Assets;
use crate::constants::*;
use crate::game::Phase;

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
    area: Rect,
) -> GameResult {
    match phase {
        Phase::Playing => Ok(()),
        Phase::Ready => draw_centered(
            ctx,
            canvas,
            "CLICK TO LAUNCH",
            SCREEN_H * 0.62,
            30.0,
            Color::new(1.0, 1.0, 1.0, 0.75),
        ),
        Phase::GameOver | Phase::Win => {
            canvas.draw(
                &Quad,
                DrawParam::new().dest_rect(area).color(OVERLAY_DIM_COLOR),
            );
            let (title, color) = if phase == Phase::Win {
                ("YOU WIN!", ROW_COLORS[2])
            } else {
                ("GAME OVER", ROW_COLORS[0])
            };
            draw_centered(ctx, canvas, title, SCREEN_H * 0.38, 76.0, color)?;
            draw_centered(
                ctx,
                canvas,
                &format!("FINAL SCORE {score}"),
                SCREEN_H * 0.50,
                34.0,
                Color::WHITE,
            )?;
            draw_centered(
                ctx,
                canvas,
                "CLICK TO RESTART",
                SCREEN_H * 0.60,
                26.0,
                Color::new(1.0, 1.0, 1.0, 0.7),
            )
        }
    }
}

fn draw_centered(
    ctx: &Context,
    canvas: &mut Canvas,
    s: &str,
    y_center: f32,
    font_px: f32,
    color: Color,
) -> GameResult {
    let mut text = Text::new(s);
    text.set_scale(PxScale::from(font_px));
    let dims = text.measure(ctx)?;
    canvas.draw(
        &text,
        DrawParam::new()
            .dest(Vec2::new(
                (SCREEN_W - dims.x) / 2.0,
                y_center - dims.y / 2.0,
            ))
            .color(color),
    );
    Ok(())
}

fn fill(canvas: &mut Canvas, rect: Rect, color: Color) {
    canvas.draw(&Quad, DrawParam::new().dest_rect(rect).color(color));
}
