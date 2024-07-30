use crate::*;

const SNAKE_POSITION: Point2<f32> = Point2::new(0.5, 0.5);
const SNAKE_ROTATION: f32 = 0.0;
const SNAKE_SPEED: f32 = 0.002;

#[derive(Debug, Clone)]
pub struct Board {
    pub(crate) apple: Apple,
    pub(crate) snake: Snake,
}

impl Board {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        let apple = Apple::random(rng);
        let snake = Snake::new(SNAKE_POSITION, SNAKE_ROTATION, SNAKE_SPEED);

        Self { apple, snake }
    }

    pub fn apple(&self) -> &Apple {
        &self.apple
    }

    pub fn snake(&self) -> &Snake {
        &self.snake
    }
}
