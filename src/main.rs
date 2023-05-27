use ggez::{ContextBuilder, event, GameResult};
use ggez::conf::{WindowMode, WindowSetup};
use crate::constants::SCREEN_SIZE;
use crate::game::Game;

mod snake;
mod food;
mod game;
mod sounds;
mod constants;

const GAME_ID: &str = "Snake";
const AUTHOR: &str = "BPoisson";

fn main() -> GameResult {
    let (ctx, event_loop) = ContextBuilder::new(&GAME_ID, &AUTHOR)
        .window_setup(WindowSetup::default().title(&GAME_ID))
        .window_mode(WindowMode::default().dimensions(SCREEN_SIZE.0 as f32, SCREEN_SIZE.1 as f32))
        .add_resource_path("resources")
        .build()?;

    let game: Game = Game::new(&ctx);

    event::run(ctx, event_loop, game);
}
