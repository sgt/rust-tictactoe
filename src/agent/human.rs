use crate::game::Position;

use crate::game::Board;

use super::Agent;

pub struct HumanAgent;

impl Agent for HumanAgent {
    fn next_move(&self, board: &Board) -> Option<Position> {
        let positions = board.available_moves();
        if positions.is_empty() {
            return None;
        }
        loop {
            println!("Enter a move (1-9): ");
            let input: usize = text_io::read!();
            if positions.contains(&Position(input)) {
                return Some(Position(input));
            }
        }
    }
}
