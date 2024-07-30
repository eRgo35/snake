mod args;
mod snake_game;

use args::Args;
use clap::Parser;
use clearscreen::clear;
use snake_game::SnakeGame;
use std::{thread, time::Duration};

fn main() {
    let args = Args::parse();
    let mut game = SnakeGame::new(args.fps);

    loop {
        clear().expect("failed to clear screen");
        game.render();
        thread::sleep(Duration::from_millis(1000 / args.fps as u64));
    }
}
