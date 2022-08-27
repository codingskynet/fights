use std::io::{self, Write};

use fights::envs::BaseEnv;
use fights::puoribor;
use text_io::scan;

#[test]
fn run_121_play() {
    let mut state = puoribor::Env::initialize_state();

    let mut iter = 0;
    while state.is_win() == -1 {
        let agent_id = iter % 2;

        print!("\x1b[1;1H");
        println!("{}", state);
        println!("What do you do next, player {}?", agent_id);

        // print next command helper
        println!();
        println!("You can enter 'action_type position_x position_y'");
        println!("Move(0): Move to specific position(absolute).");
        println!("PlaceWall(1(horizontal), 2(vertical)): Place wall horizontal(left position) or vertical(top position).");
        println!("RotateSection(2): Rotate the 4x4 local board w/o pawns. Enter the left-top position of the local board.");

        loop {
            print!(": ");
            io::stdout().flush().unwrap();

            let (action_type, pos_x, pos_y): (usize, usize, usize);
            scan!("{} {} {}", action_type, pos_x, pos_y);
            let action = puoribor::Action::new(action_type, (pos_x, pos_y));

            match puoribor::Env::step(state.clone(), agent_id, action) {
                Ok(new_state) => {
                    state = new_state;
                    break;
                }
                Err(err_reason) => println!("{} Try again!", err_reason),
            }
        }

        iter += 1;
    }

    println!("iters: {}", iter);
    if state.is_win() == 0 {
        println!("The player 0 is won!");
    } else {
        println!("The player 1 is won!");
    }
}
