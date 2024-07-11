use ggez::graphics::{Canvas, Color, DrawParam, Quad, Rect};

use crate::constants::*;

pub fn build_grid() -> Vec<Brick> {
    let brick_w = (SCREEN_W - 2.0 * BRICK_SIDE_MARGIN - (BRICK_COLS as f32 - 1.0) * BRICK_GAP)
        / BRICK_COLS as f32;

    ROW_COLORS
        .iter()
        .enumerate()
        .flat_map(|(row, &color)| {
            (0..BRICK_COLS).map(move |col| Brick {
                rect: Rect::new(
                    BRICK_SIDE_MARGIN + col as f32 * (brick_w + BRICK_GAP),
                    BRICK_TOP + row as f32 * (BRICK_H + BRICK_GAP),
                    brick_w,
                    BRICK_H,
                ),
                color,
                points: ((BRICK_ROWS - row) * 10) as u32,
                alive: true,
            })
        })
        .collect()
}

pub struct Brick {
    pub rect: Rect,
    pub color: Color,
    pub points: u32,
    pub alive: bool,
}

impl Brick {
    pub fn draw(&self, canvas: &mut Canvas) {
        let c = self.color;
        let shade = Color::new(c.r * 0.45, c.g * 0.45, c.b * 0.45, 1.0);
        canvas.draw(&Quad, DrawParam::new().dest_rect(self.rect).color(shade));
        let face = Rect::new(
            self.rect.x,
            self.rect.y,
            self.rect.w - 3.0,
            self.rect.h - 3.0,
        );
        canvas.draw(&Quad, DrawParam::new().dest_rect(face).color(c));
    }
}
