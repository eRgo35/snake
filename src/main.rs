mod args;
mod snake_game;

use args::Args;
use clap::Parser;
use clearscreen::clear;
use snake_game::SnakeGame;
use std::{f32::consts::{FRAC_PI_2, PI}, thread, time::Duration};
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{enable_raw_mode, disable_raw_mode},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let mut game = SnakeGame::new(args.tps);

    enable_raw_mode()?;

    loop {
        if event::poll(Duration::from_millis(0))? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Up | KeyCode::Char('w') => game.rotate(FRAC_PI_2),
                    KeyCode::Down | KeyCode::Char('s') => game.rotate(-FRAC_PI_2),
                    KeyCode::Left | KeyCode::Char('a') => game.rotate(PI),
                    KeyCode::Right | KeyCode::Char('d') => game.rotate(0.0),
                    KeyCode::Char('r') => game.reset(),
                    KeyCode::Char('q') => break,
                    _ => {}
                }
            }
        }

        clear().expect("failed to clear screen");
        game.render();
        thread::sleep(Duration::from_millis(1000 / args.tps as u64));
    }

    // Disable raw mode before exiting
    disable_raw_mode()?;

    Ok(())
}