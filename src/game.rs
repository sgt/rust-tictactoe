use std::fmt::Display;

// Position on board from 1 to 9, starting with the top left corner
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Position(usize);
impl Position {
    pub fn as_idx(&self) -> usize {
        self.0 - 1
    }

    pub fn from_idx(idx: usize) -> Self {
        Position(idx + 1)
    }
}

#[derive(Debug, PartialEq)]
pub enum Player {
    X,
    O,
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::X => write!(f, "X"),
            Self::O => write!(f, "O"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum State {
    Impossible,
    Tie,
    Won(Player),
    TurnOf(Player),
}

#[derive(Clone, Copy, PartialEq)]
pub enum Cell {
    Empty,
    X,
    O,
}

impl Cell {
    pub fn from(player: &Player) -> Self {
        match player {
            Player::X => Cell::X,
            Player::O => Cell::O,
        }
    }
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

    pub fn play(&mut self) {
        println!("{}", self);
        match self.state() {
            State::Impossible => {
                panic!("Error: impossible state, quitting");
            }
            State::Tie => {
                println!("It's a tie!");
            }
            State::TurnOf(player) => {
                print!("Enter the position for player {player}:");
                let pos: usize = text_io::read!();
                self.turn(Position(pos));
                self.play()
            }
            State::Won(player) => {
                println!("Player {player} won!");
            }
        }
    }

    pub fn state(&self) -> State {
        let x_count = self.0.iter().filter(|&cell| *cell == Cell::X).count();
        let o_count = self.0.iter().filter(|&cell| *cell == Cell::O).count();
        if x_count > o_count + 1 || o_count > x_count {
            State::Impossible
        } else if Self::TRIPLETS
            .iter()
            .any(|triplet| triplet.iter().all(|&p| self.get(p) == Cell::X))
        {
            State::Won(Player::X)
        } else if Self::TRIPLETS
            .iter()
            .any(|triplet| triplet.iter().all(|&p| self.get(p) == Cell::O))
        {
            State::Won(Player::O)
        } else if self.0.iter().all(|&cell| cell != Cell::Empty) {
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
                    self.set(pos, Cell::from(&player));
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
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut board = String::new();
        for (i, cell) in self.0.iter().enumerate() {
            match cell {
                Cell::Empty => board.push('.'),
                Cell::X => board.push('X'),
                Cell::O => board.push('O'),
            }
            if i % 3 == 2 {
                board.push('\n');
            }
        }
        write!(f, "{board}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state() {
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
