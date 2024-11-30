use std::fmt::Display;

use owo_colors::OwoColorize;
use tabled::{builder::Builder, settings::Style};

// Position on board from 1 to 9, starting with the top left corner
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Position(pub usize);
impl Position {
    pub fn as_idx(&self) -> usize {
        self.0 - 1
    }

    pub fn from_idx(idx: usize) -> Self {
        Position(idx + 1)
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord)]
pub enum Player {
    X,
    O,
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::X => write!(f, "{}", "X".blue()),
            Self::O => write!(f, "{}", "O".red()),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum State {
    Impossible,
    Tie,
    Won(Player),
    TurnOf(Player),
}

#[derive(Clone, Copy, PartialEq)]
pub enum Cell {
    Empty,
    Marked(Player),
}

pub struct Board([Cell; 9]);

impl Board {
    const TRIPLETS: [[Position; 3]; 8] = [
        // rows
        [Position(1), Position(2), Position(3)],
        [Position(4), Position(5), Position(6)],
        [Position(7), Position(8), Position(9)],
        // columns
        [Position(1), Position(4), Position(7)],
        [Position(2), Position(5), Position(8)],
        [Position(3), Position(6), Position(9)],
        // diagonals
        [Position(1), Position(5), Position(9)],
        [Position(3), Position(5), Position(7)],
    ];

    pub fn get(&self, pos: Position) -> Cell {
        self.0[pos.as_idx()]
    }

    pub fn set(&mut self, pos: Position, cell: Cell) {
        self.0[pos.as_idx()] = cell;
    }

    pub fn new() -> Self {
        Board([Cell::Empty; 9])
    }

    fn has_won(&self, player: Player) -> bool {
        Self::TRIPLETS
            .iter()
            .any(|triplet| triplet.iter().all(|&p| self.get(p) == Cell::Marked(player)))
    }

    fn is_tie(&self) -> bool {
        self.0.iter().all(|&cell| cell != Cell::Empty)
    }

    pub fn state(&self) -> State {
        let x_count = self
            .0
            .iter()
            .filter(|&cell| *cell == Cell::Marked(Player::X))
            .count();
        let o_count = self
            .0
            .iter()
            .filter(|&cell| *cell == Cell::Marked(Player::O))
            .count();
        if x_count > o_count + 1 || o_count > x_count {
            State::Impossible
        } else if self.has_won(Player::X) {
            State::Won(Player::X)
        } else if self.has_won(Player::O) {
            State::Won(Player::O)
        } else if self.is_tie() {
            State::Tie
        } else if x_count == o_count {
            State::TurnOf(Player::X)
        } else {
            State::TurnOf(Player::O)
        }
    }

    /// Make a turn for the next player.
    pub fn turn(&mut self, pos: Position) -> bool {
        match self.state() {
            State::TurnOf(player) => {
                let cell = self.get(pos);
                if cell != Cell::Empty {
                    false
                } else {
                    self.set(pos, Cell::Marked(player));
                    true
                }
            }
            state => panic!("trying to perform a turn when state is {state:?}"),
        }
    }

    pub fn available_moves(&self) -> Vec<Position> {
        self.0
            .iter()
            .enumerate()
            .filter_map(|(i, &cell)| match cell == Cell::Empty {
                true => Some(Position::from_idx(i)),
                false => None,
            })
            .collect()
    }

    fn table_row(&self, row: usize) -> impl Iterator<Item = String> + '_ {
        self.0[row * 3..3 + row * 3]
            .iter()
            .enumerate()
            .map(move |(i, cell)| match cell {
                Cell::Empty => Position::from_idx(i + row * 3).to_string(),
                Cell::Marked(player) => player.to_string(),
            })
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut builder = Builder::default();
        (0..=2).for_each(|i| builder.push_record(self.table_row(i)));
        write!(f, "{}", builder.build().with(Style::modern_rounded()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_board() {
        let mut board = Board::new();
        assert_eq!(board.state(), State::TurnOf(Player::X));
        assert!(board.turn(Position(5)));
        assert_eq!(board.state(), State::TurnOf(Player::O));
        assert!(!board.turn(Position(5))); // can't play the same cell twice
        assert!(board.turn(Position(1)));
        assert_eq!(
            board.available_moves(),
            vec![
                Position(2),
                Position(3),
                Position(4),
                Position(6),
                Position(7),
                Position(8),
                Position(9)
            ]
        );
        // proceed to winning the game
        assert!(board.turn(Position(2)));
        assert!(board.turn(Position(9)));
        assert!(board.turn(Position(8)));
        assert_eq!(board.state(), State::Won(Player::X));
    }
}
