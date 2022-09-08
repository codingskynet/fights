use std::{thread, time::Duration};

use fights::{envs::BaseEnv, puoribor};

fn play_from_log(actions: Vec<(u8, u8, u8)>, delay: u64) {
    let mut state = puoribor::Env::initialize_state();

    for (iter, action) in actions.iter().enumerate() {
        thread::sleep(Duration::from_millis(delay));

        let agent_id = iter % 2;

        print!("{esc}[2J{esc}[1;1H", esc = 27 as char); // clear terminal
        println!("{}", state);
        println!("Player {} will do: {:?}", agent_id, action);

        let action = puoribor::Action::new(action.0, (action.1, action.2));

        match puoribor::Env::step(state.clone(), agent_id, action) {
            Ok(new_state) => {
                state = new_state;
            }
            Err(err_reason) => unreachable!("{}", err_reason),
        }
    }

    print!("{esc}[2J{esc}[1;1H", esc = 27 as char); // clear terminal
    println!("{}", state);
    if state.is_win() == 0 {
        println!("The player 0 is won!");
    } else {
        println!("The player 1 is won!");
    }
}

#[test]
fn replay() {
    let actions = vec![
        (1, 3, 1),
        (3, 1, 0),
        (1, 4, 2),
        (1, 4, 8),
        (1, 2, 1),
        (1, 0, 1),
        (2, 6, 1),
        (2, 6, 4),
        (0, 5, 0),
        (2, 4, 0),
        (1, 5, 6),
        (3, 2, 1),
        (1, 2, 4),
        (3, 5, 5),
        (2, 1, 1),
        (0, 3, 8),
        (1, 2, 1),
        (0, 4, 8),
        (1, 4, 7),
        (0, 3, 8),
        (2, 2, 4),
        (0, 3, 7),
        (0, 6, 0),
        (0, 3, 6),
        (0, 6, 1),
        (0, 3, 5),
        (0, 6, 2),
        (0, 4, 5),
        (0, 6, 3),
        (0, 5, 5),
        (0, 6, 4),
        (0, 5, 4),
        (0, 6, 5),
        (0, 5, 3),
        (0, 6, 6),
        (0, 6, 3),
        (0, 6, 7),
        (0, 6, 2),
        (0, 6, 8),
    ];

    play_from_log(actions, 500);
}
