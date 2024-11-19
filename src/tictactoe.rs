use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
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

#[derive(Debug, PartialEq, Eq)]
pub enum State {
    Impossible,
    Tie,
    Won(Player),
    TurnOf(Player),
}

#[derive(Clone, Copy, PartialEq, Eq)]
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

pub struct TicTacToe {
    board: [Cell; 9],
}

impl TicTacToe {
    pub fn new() -> Self {
        TicTacToe {
            board: [Cell::Empty; 9],
        }
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
            State::TurnOf(_) => {
                print!("Enter the coordinates separated by space:");
                let col: usize = text_io::read!();
                let row: usize = text_io::read!();
                self.turn(col, row);
                self.play()
            }
            State::Won(player) => {
                println!("Player {player} won!");
            }
        }
    }

    pub fn state(&self) -> State {
        let x_count = self.board.iter().filter(|&cell| *cell == Cell::X).count();
        let o_count = self.board.iter().filter(|&cell| *cell == Cell::O).count();
        if x_count > o_count + 1 || o_count > x_count {
            State::Impossible
        } else if self
            .triplets()
            .iter()
            .any(|triplet| triplet.iter().all(|&cell| cell == Cell::X))
        {
            State::Won(Player::X)
        } else if self
            .triplets()
            .iter()
            .any(|triplet| triplet.iter().all(|&cell| cell == Cell::O))
        {
            State::Won(Player::O)
        } else if self.board.iter().all(|&cell| cell != Cell::Empty) {
            State::Tie
        } else if x_count == o_count {
            State::TurnOf(Player::X)
        } else {
            State::TurnOf(Player::O)
        }
    }

    pub fn turn(&mut self, col: usize, row: usize) -> bool {
        match self.state() {
            State::TurnOf(player) => {
                let cell = &mut self.board[Self::idx(col, row)];
                if *cell != Cell::Empty {
                    false
                }else {
                *cell = Cell::from(&player);
                true
                }
            }
            state => panic!("trying to perform a turn when state is {state:?}"),
        }
    }

    fn idx(col: usize, row: usize) -> usize {
        row * 3 + col
    }

    // /// returns (col, row)
    // fn coords(idx: usize) -> (usize, usize) {
    //     (idx % 3, idx / 3)
    // }

    fn triplets(&self) -> Vec<Vec<Cell>> {
        vec![
            // rows
            vec![self.board[0], self.board[1], self.board[2]],
            vec![self.board[3], self.board[4], self.board[5]],
            vec![self.board[6], self.board[7], self.board[8]],
            // cols
            vec![self.board[0], self.board[3], self.board[6]],
            vec![self.board[1], self.board[4], self.board[7]],
            vec![self.board[2], self.board[5], self.board[8]],
            // diags
            vec![self.board[0], self.board[4], self.board[8]],
            vec![self.board[2], self.board[4], self.board[6]],
        ]
    }
}

impl Display for TicTacToe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut board = String::new();
        for (i, cell) in self.board.iter().enumerate() {
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
        let mut game = TicTacToe::new();
        assert_eq!(game.state(), State::TurnOf(Player::X));
        assert!(game.turn(1, 1));
        assert_eq!(game.state(), State::TurnOf(Player::O));
        assert!(!game.turn(1, 1)); // can't play the same cell twice
        assert!(game.turn(0, 0));
    }
}
