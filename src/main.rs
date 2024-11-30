use std::{collections::HashMap, fmt::Display};

use agent::{Agent, RandomAgent};
use clap::Parser;
use game::{Board, Player, State};

mod agent;
mod game;

#[derive(Parser)]
#[command(version, about, long_about=None)]
struct Args {}

struct PlayOptions {
    print_board_every_turn: bool,
    print_final_board: bool,
    print_game_outcome: bool,
}

fn play<AgentX, AgentY>(
    board: &mut Board,
    player_x: &AgentX,
    player_o: &AgentY,
    options: &PlayOptions,
) -> State
where
    AgentX: Agent,
    AgentY: Agent,
{
    match board.state() {
        State::Impossible => {
            panic!("Error: impossible state, quitting");
        }
        State::Tie => {
            if options.print_final_board {
                println!("{board}");
            }
            if options.print_game_outcome {
                println!("It's a tie!");
            }
        }
        State::TurnOf(player) => {
            if options.print_board_every_turn {
                println!("{board}");
            }
            let player: &dyn Agent = match player {
                Player::X => player_x,
                Player::O => player_o,
            };
            if let Some(pos) = player.next_move(board) {
                board.turn(pos);
                play(board, player_x, player_o, options);
            } else {
                panic!("Error: no available moves, quitting");
            }
        }
        State::Won(player) => {
            if options.print_final_board {
                println!("{board}");
            }
            if options.print_game_outcome {
                println!("Player {player} won!");
            }
        }
    }
    board.state()
}

/*
Desired cli syntax:

$ tictactoe stats -n 1000 -x random -o random
$ tictactoe play -x random -o human
*/

struct GameStats(HashMap<State, usize>);
impl Display for GameStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result_tuples = self.0.iter().collect::<Vec<_>>();
        result_tuples.sort();
        write!(f, "{:?}", result_tuples)
    }
}

fn game_stats(iterations: usize) -> GameStats {
    let mut results = HashMap::new();

    for _ in 0..iterations {
        let mut board = Board::new();
        let state = play(
            &mut board,
            &RandomAgent,
            &RandomAgent,
            &PlayOptions {
                print_board_every_turn: false,
                print_final_board: false,
                print_game_outcome: false,
            },
        );
        *results.entry(state).or_insert(0) += 1;
    }

    GameStats(results)
}

fn main() {
    // let args = Args::parse();
    println!("{}", game_stats(1000));
}
