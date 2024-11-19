use tictactoe::TicTacToe;

mod tictactoe;

fn main() {
    let mut game = TicTacToe::new();
    game.play();
}
