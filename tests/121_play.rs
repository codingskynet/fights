use std::io::{self, Write};

use fights::envs::BaseEnv;
use fights::puoribor;
use ndarray::Array2;
use text_io::scan;

#[inline]
pub fn i(x: (u8, u8)) -> (usize, usize) {
    (x.0 as usize, x.1 as usize)
}

fn get_all_available_move(state: &puoribor::State, agent_id: usize) -> Vec<(u8, u8)> {
    (0..9)
        .flat_map(|y| (0..9).map(move |x| (x, y)))
        .filter_map(|pos| {
            if puoribor::Env::step(state.clone(), agent_id, puoribor::Action::new(0, pos)).is_ok() {
                Some(pos)
            } else {
                None
            }
        })
        .collect::<Vec<(u8, u8)>>()
}

#[test]
fn run_121_play() {
    let mut state = puoribor::Env::initialize_state();

    let mut iter = 0;
    while state.is_win() == -1 {
        let agent_id = iter % 2;

        // print!("{esc}[2J{esc}[1;1H", esc = 27 as char); // clear terminal
        println!("{}", state);
        println!("What do you do next, player {}?", agent_id);

        // print next command helper
        println!("If you want to show command helper, just enter -1.");

        loop {
            print!(": ");
            io::stdout().flush().unwrap();

            let command_type: i32;

            scan!("{}", command_type);

            match command_type {
                -1 => {
                    println!("You can enter 'action_type position_x position_y', whose position is aboslute starting from top-left.");
                    println!("Move(0): Move to specific position.");
                    println!("PlaceWall(1(horizontal), 2(vertical)): Place wall horizontal(left position) or vertical(top position).");
                    println!("RotateSection(3): Rotate the 4x4 local board w/o pawns. Enter the left-top position of the local board.");
                    println!("Show next able moving(4): Print on next available movement on board by X mark.");
                }
                0..=3 => {
                    let (pos_x, pos_y): (u8, u8);
                    scan!("{} {}", pos_x, pos_y);
                    let action = puoribor::Action::new(command_type as u8, (pos_x, pos_y));

                    match puoribor::Env::step(state.clone(), agent_id, action) {
                        Ok(new_state) => {
                            state = new_state;
                            break;
                        }
                        Err(err_reason) => println!("{} Try again!", err_reason),
                    }
                }
                4 => {
                    let pos_list = get_all_available_move(&state, agent_id);

                    let mut marker_board = Array2::zeros([9, 9]);

                    for pos in pos_list {
                        marker_board[i(pos)] = 1;
                    }

                    println!("{}", state.display_with(Some(("X", marker_board))));
                }
                _ => {
                    println!("Invalid command type. Please re-enter.");
                }
            }
        }

        iter += 1;
    }

    print!("{esc}[2J{esc}[1;1H", esc = 27 as char); // clear terminal
    println!("{}", state);
    println!("iters: {}", iter);
    if state.is_win() == 0 {
        println!("The player 0 is won!");
    } else {
        println!("The player 1 is won!");
    }
}
