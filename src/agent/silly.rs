use crate::game::Position;
use rand::prelude::*;

use super::Agent;

/// Making random moves.
pub struct SillyAgent;

impl Agent for SillyAgent {
    fn next_move(&self, board: &crate::game::Board) -> Option<Position> {
        let positions = board.available_moves();
        if positions.is_empty() {
            None
        } else {
            Some(positions[random::<usize>() % positions.len()])
        }
    }
}
