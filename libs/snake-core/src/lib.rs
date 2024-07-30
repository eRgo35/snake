mod apple;
mod board;
mod snake;

use self::{apple::*, board::*, snake::*};

use nalgebra::{distance, wrap, Point2, Rotation2, Vector2};
use rand::{Rng, RngCore};

#[derive(Debug, Clone)]
pub struct Game {
    board: Board,
    points: usize,
    tick: usize,
}

impl Game {
    pub fn new(rng: &mut dyn RngCore) -> Self {
        let board = Board::random(rng);

        Self { board, points: 0, tick: 0 }
    }

    pub fn board(&self) -> &Board {
        &self.board
    }

    pub fn points(&self) -> usize {
        self.points
    }

    pub fn tick(&self) -> usize {
        self.tick
    }

    pub fn step(&mut self, rng: &mut dyn RngCore) {
        self.process_collisions(rng);
        self.process_movements();

        self.tick += 1;
    }

    pub fn reset(&mut self, rng: &mut dyn RngCore) {
        self.points = 0;
        self.tick = 0;

        self.board = Board::random(rng);
    }

    fn process_collisions(&mut self, rng: &mut dyn RngCore) {
        let snake = &mut self.board.snake;
        let apple = &mut self.board.apple;

        let distance = distance(&snake.position(), &apple.position());

        if distance < 0.01 {
            apple.position = rng.gen();
        }
    }

    fn process_movements(&mut self) {
        let snake = &mut self.board.snake;

        snake.position += snake.rotation * Vector2::new(0.0, snake.speed);

        snake.position.x = wrap(snake.position.x, 0.0, 1.0);
        snake.position.y = wrap(snake.position.y, 0.0, 1.0);
    }

}
