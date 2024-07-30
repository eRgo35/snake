use clap::Parser;

#[derive(Parser)]
#[command(
    name = "Snake",
    author = "Michał Czyż",
    version = "1.0.0",
    about = "A simple snake game",
    long_about = None
)]
pub struct Args {
    #[arg(long, default_value_t = 120)]
    pub tps: usize,
}