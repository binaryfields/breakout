mod assets;
mod ball;
mod brick;
mod collision;
mod constants;
mod game;
mod paddle;
mod ui;

use ggez::conf::{WindowMode, WindowSetup};
use ggez::{event, ContextBuilder, GameResult};

use crate::constants::{SCREEN_H, SCREEN_W};
use crate::game::Game;

fn main() -> GameResult {
    let (mut ctx, event_loop) = ContextBuilder::new("breakout", "sebby")
        .window_setup(WindowSetup::default().title("Breakout"))
        .window_mode(
            WindowMode::default()
                .dimensions(SCREEN_W, SCREEN_H)
                .resizable(false),
        )
        .build()?;
    let game = Game::new(&mut ctx)?;
    event::run(ctx, event_loop, game)
}
