use ggez::graphics::Color;

pub const SCREEN_W: f32 = 960.0;
pub const SCREEN_H: f32 = 720.0;

pub const PADDLE_W: f32 = 120.0;
pub const PADDLE_H: f32 = 16.0;
pub const PADDLE_Y_OFFSET: f32 = 40.0;
pub const PADDLE_SPEED_PPS: f32 = 700.0;

pub const BALL_RADIUS: f32 = 8.0;
pub const BALL_SPEED_START_PPS: f32 = 420.0;

pub const PLAYFIELD_TOP: f32 = 50.0;

pub const BG_COLOR: Color = Color {
    r: 0.07,
    g: 0.07,
    b: 0.11,
    a: 1.0,
};
pub const PADDLE_COLOR: Color = Color {
    r: 0.86,
    g: 0.88,
    b: 0.92,
    a: 1.0,
};
