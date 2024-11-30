use crate::game::{Position, Board};

pub mod human;
pub mod random;

pub use random::RandomAgent;

pub trait Agent {
    fn next_move(&self, board: &Board) -> Option<Position>;
}
