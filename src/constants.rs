use ggez::glam::Vec2;
use ggez::graphics::Color;

pub const SCREEN_W: f32 = 960.0;
pub const SCREEN_H: f32 = 720.0;

pub const PADDLE_W: f32 = 120.0;
pub const PADDLE_H: f32 = 16.0;
pub const PADDLE_Y_OFFSET: f32 = 40.0;
pub const PADDLE_SPEED_PPS: f32 = 700.0;

pub const BALL_RADIUS: f32 = 8.0;
pub const BALL_SPEED_START_PPS: f32 = 420.0;
pub const BALL_SPEED_MAX_PPS: f32 = 640.0;
pub const BALL_SPEED_INCREMENT_PPS: f32 = 6.0;
pub const MAX_BOUNCE_ANGLE_DEG: f32 = 60.0;
pub const LAUNCH_SPREAD_DEG: f32 = 30.0;

pub const START_LIVES: u32 = 3;

pub const MAX_FRAME_DT: f32 = 1.0 / 30.0;

pub const HUD_LINE_H: f32 = 1.5;
pub const HUD_TEXT_PX: f32 = 26.0;
pub const HUD_SCORE_POS: Vec2 = Vec2::new(20.0, 13.0);
pub const LIFE_SPACING: f32 = 26.0;
pub const LIFE_Y: f32 = 25.0;
pub const LIFE_SCALE: f32 = 0.75;
pub const WALL_THICKNESS: f32 = 1.5;
pub const PROMPT_PULSE_BASE: f32 = 0.55;
pub const PROMPT_PULSE_AMP: f32 = 0.30;
pub const PROMPT_PULSE_HZ: f32 = 3.0;

pub const STAR_COUNT: usize = 60;
pub const TRAIL_LEN: usize = 14;
pub const TRANSITION_TIME_SEC: f32 = 0.25;

pub const BRICK_ROWS: usize = 6;
pub const BRICK_COLS: usize = 10;
pub const BRICK_H: f32 = 24.0;
pub const BRICK_GAP: f32 = 6.0;
pub const BRICK_SIDE_MARGIN: f32 = 40.0;
pub const BRICK_TOP: f32 = 90.0;

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

pub const ROW_COLORS: [Color; BRICK_ROWS] = [
    Color {
        r: 0.94,
        g: 0.25,
        b: 0.30,
        a: 1.0,
    },
    Color {
        r: 1.00,
        g: 0.55,
        b: 0.15,
        a: 1.0,
    },
    Color {
        r: 1.00,
        g: 0.84,
        b: 0.20,
        a: 1.0,
    },
    Color {
        r: 0.30,
        g: 0.85,
        b: 0.42,
        a: 1.0,
    },
    Color {
        r: 0.34,
        g: 0.70,
        b: 1.00,
        a: 1.0,
    },
    Color {
        r: 0.66,
        g: 0.46,
        b: 0.96,
        a: 1.0,
    },
];
pub const HUD_LINE_COLOR: Color = Color {
    r: 1.0,
    g: 1.0,
    b: 1.0,
    a: 0.15,
};
pub const LIVES_COLOR: Color = Color {
    r: 1.0,
    g: 1.0,
    b: 1.0,
    a: 0.9,
};
pub const OVERLAY_DIM_COLOR: Color = Color {
    r: 0.0,
    g: 0.0,
    b: 0.0,
    a: 0.6,
};
pub const WALL_COLOR: Color = Color {
    r: 1.0,
    g: 1.0,
    b: 1.0,
    a: 0.15,
};
pub const BALL_GLOW_COLOR: Color = Color {
    r: 0.65,
    g: 0.80,
    b: 1.0,
    a: 0.10,
};
pub const TRAIL_COLOR: Color = Color {
    r: 0.55,
    g: 0.75,
    b: 1.0,
    a: 1.0,
};
