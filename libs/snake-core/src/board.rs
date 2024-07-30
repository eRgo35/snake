use crate::*;

const SNAKE_POSITION: Point2<f32> = Point2::new(0.5, 0.5);
const SNAKE_SPEED: f32 = 0.002;

#[derive(Debug, Clone)]
pub struct Board {
    pub(crate) apple: Apple,
    pub(crate) snake: Vec<Snake>,
}

impl Board {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        let apple = Apple::random(rng);
        let snake = Snake::new(SNAKE_POSITION, Rotation2::new(0.0), SNAKE_SPEED);

        Self { apple, snake: vec![snake] }
    }

    pub fn apple(&self) -> &Apple {
        &self.apple
    }

    pub fn snake(&self) -> &Vec<Snake> {
        &self.snake
    }

    pub fn grow_snake(&mut self) {
        let new_position = self.snake.last().unwrap().position() + Vector2::new(0.0, 0.02);

        self.snake.push(Snake::new(
            new_position,
            self.snake.last().unwrap().rotation(),
            self.snake.last().unwrap().speed(),
        ));
    }
}
