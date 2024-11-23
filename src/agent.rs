use crate::game::{Position, Board};

pub mod minimax;
pub mod silly;
pub use minimax::MinimaxAgent;
pub use silly::SillyAgent;

pub trait Agent {
    fn next_move(&self, board: &Board) -> Option<Position>;
}
