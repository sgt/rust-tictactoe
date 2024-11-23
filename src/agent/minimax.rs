use crate::game::Position;

use super::Agent;

pub struct MinimaxAgent;

impl Agent for MinimaxAgent {
    fn next_move(&self, board: &crate::game::Board) -> Option<Position> {
        todo!()
    }
}
