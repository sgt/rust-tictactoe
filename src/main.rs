use std::collections::HashMap;

use agent::{Agent, SillyAgent};
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
fn main() {
    let args = Args::parse();
    let mut results: HashMap<State, usize> = HashMap::new();
    for _ in 0..1_000 {
        let mut board = Board::new();
        let state = play(
            &mut board,
            &SillyAgent,
            &SillyAgent,
            &PlayOptions {
                print_board_every_turn: false,
                print_final_board: false,
                print_game_outcome: false,
            },
        );
        *results.entry(state).or_insert(0) += 1;
    }
    let mut result_tuples = results.into_iter().collect::<Vec<_>>();
    result_tuples.sort();
    println!("{:?}", result_tuples);
}
