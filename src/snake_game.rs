use crossterm::style::Stylize;
use lib_snake_core::*;
use nalgebra::Point2;
use rand::prelude::*;

const RESOLUTION: usize = 32;
const TPS: usize = 120;

struct Textures {
    grass: String,
    apple: String,
    snake_head: String,
    snake_body: String,
}

impl Textures {
    fn new() -> Self {
        Self {
            grass: "##".green().to_string(),
            apple: "@@".red().to_string(),
            snake_head: "[]".blue().to_string(),
            snake_body: "()".blue().to_string(),
        }
    }
}

pub struct SnakeGame {
    rng: ThreadRng,
    game: Game,
    tps: usize,
    textures: Textures,
}

impl SnakeGame {
    pub fn new(tps: usize) -> Self {
        let mut rng = thread_rng();
        let game = Game::new(&mut rng);

        Self { rng, game, tps, textures: Textures::new() }
    }

    pub fn rotate(&mut self, rotation: f32) {
        self.game.rotate(rotation);
    }

    pub fn reset(&mut self) {
        self.game.reset(&mut self.rng);
    }

    pub fn render(&mut self) {
        self.render_header();
        self.render_board();

        self.game.step(&mut self.rng);
    }

    fn render_header(&self) {
        let time = self.game.tick() / self.tps;
        let over = match self.game.over() {
            true => format!(" {} ::", "GAME OVER".red()),
            false => "".to_string(),
        };

        print!(":: Snake :: {} pts :: {} secs ::{}\r\n", self.game.points(), time, over);
    }

    fn render_board(&self) {
        let rendered_board: Vec<Vec<String>> = vec![vec![self.textures.grass.clone(); RESOLUTION]; RESOLUTION];

        let board = self.game.board();
        let apple = &board.apple();
        let snake = &board.snake();

        for (x, row) in rendered_board.iter().enumerate().take(RESOLUTION) {
            for (y, pixel) in row.iter().enumerate().take(RESOLUTION) {
                let apple_position = self.scaled_position(apple.position());
                let snake_head_position = self.scaled_position(snake[0].position());
                let snake_position: Vec<(usize, usize)> = snake.iter().map(|s| self.scaled_position(s.position())).collect();

                if snake_head_position.0 == x && snake_head_position.1 == y {
                    print!("{}", self.textures.snake_head);
                    continue;
                }

                if !snake_position.is_empty() {    
                    for snake_body_position in &snake_position {
                        if snake_body_position.0 == x && snake_body_position.1 == y {
                            print!("{}", self.textures.snake_body);
                        }
                    }
                }
                
                if apple_position.0 == x && apple_position.1 == y {
                    print!("{}", self.textures.apple);
                    continue;
                }

                print!("{}", pixel)
            }
            println!("\r");
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
        Self::new(TPS)
    }
}
