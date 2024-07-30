mod apple;
mod board;
mod snake;

use self::{apple::*, board::*, snake::*};

use nalgebra::{distance, Point2, Rotation2, Vector2};
use rand::{Rng, RngCore};

#[derive(Debug, Clone)]
pub struct Game {
    board: Board,
    points: usize,
    tick: usize,
    over: bool,
}

impl Game {
    pub fn new(rng: &mut dyn RngCore) -> Self {
        let board = Board::random(rng);

        Self {
            board,
            points: 0,
            tick: 0,
            over: false,
        }
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

    pub fn over(&self) -> bool {
        self.over
    }

    pub fn rotate(&mut self, rotation: f32) {
        for snake in &mut self.board.snake {
            snake.rotation = Rotation2::new(rotation);
        }
    }

    pub fn step(&mut self, rng: &mut dyn RngCore) {
        if self.over {
            return;    
        }

        self.process_collisions(rng);
        self.process_movements();

        self.tick += 1;
    }

    fn game_over(&mut self) {
        self.board.snake[0].speed = 0.0;

        self.over = true;
    }

    pub fn reset(&mut self, rng: &mut dyn RngCore) {
        self.points = 0;
        self.tick = 0;

        self.board = Board::random(rng);
    }

    fn process_collisions(&mut self, rng: &mut dyn RngCore) {
        let snake = &mut self.board.snake;
        let apple = &mut self.board.apple;

        let distance = distance(&snake[0].position(), &apple.position());

        if distance < 0.01 {
            apple.position = rng.gen();

            self.points += 1;

            self.board.grow_snake();
        }
    }

    fn process_movements(&mut self) {
        let snake_full = &mut self.board.snake;

        for snake in snake_full {
            snake.position += snake.rotation * Vector2::new(0.0, snake.speed);
            
            snake.position.x = snake.position.x.min(0.99).max(0.0);
            snake.position.y = snake.position.y.min(0.99).max(0.0);
            
            // if snake.position.x == 0.0 || snake.position.x == 0.99 || snake.position.y == 0.0 || snake.position.y == 0.99 {
            //     self.game_over();
            // }
        }
    }
}
