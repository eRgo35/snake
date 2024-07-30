use colored::Colorize;
use lib_snake_core::*;
use nalgebra::Point2;
use rand::prelude::*;

const RESOLUTION: usize = 32;
const FPS: usize = 60;

struct Textures {
    grass: String,
    apple: String,
    snake_head: String,
}

impl Textures {
    fn new() -> Self {
        Self {
            grass: "##".green().to_string(),
            apple: "@@".red().to_string(),
            snake_head: "[]".blue().to_string(),
        }
    }
}

pub struct SnakeGame {
    rng: ThreadRng,
    game: Game,
    fps: usize,
    textures: Textures,
}

impl SnakeGame {
    pub fn new(fps: usize) -> Self {
        let mut rng = thread_rng();
        let game = Game::new(&mut rng);

        Self { rng, game, fps, textures: Textures::new() }
    }

    pub fn render(&mut self) {
        self.render_header();
        self.render_board();

        self.game.step(&mut self.rng);
    }

    fn render_header(&self) {
        let time = self.game.tick() / self.fps;

        println!(":: Snake :: {} pts :: {} secs ::", self.game.points(), time);
    }

    fn render_board(&self) {
        let rendered_board: Vec<Vec<String>> = vec![vec![self.textures.grass.clone(); RESOLUTION]; RESOLUTION];

        let board = self.game.board();
        let apple = &board.apple();
        let snake = &board.snake();

        for (x, row) in rendered_board.iter().enumerate().take(RESOLUTION) {
            for (y, pixel) in row.iter().enumerate().take(RESOLUTION) {
                let apple_position = self.scaled_position(apple.position());
                let snake_position = self.scaled_position(snake.position());

                if snake_position.0 == x && snake_position.1 == y {
                    print!("{}", self.textures.snake_head);
                    continue;
                }
                
                if apple_position.0 == x && apple_position.1 == y {
                    print!("{}", self.textures.apple);
                    continue;
                }

                print!("{}", pixel)
            }
            println!();
        }
    }

    fn scaled_position(&self, position: Point2<f32>) -> (usize, usize) {
        (
            (position.x * RESOLUTION as f32) as usize,
            (position.y * RESOLUTION as f32) as usize,
        )
    }
}

impl Default for SnakeGame {
    fn default() -> Self {
        Self::new(FPS)
    }
}
