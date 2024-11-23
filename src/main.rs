use agent::{MinimaxAgent, SillyAgent, Agent};
use clap::Parser;
use game::Board;

mod agent;
mod game;

#[derive(Parser)]
#[command(version, about, long_about=None)]
struct Args {}

fn main() {
    let args = Args::parse();
    let mut board = Board::new();
    let agent: &dyn Agent = &SillyAgent{};
    board.play();
}
