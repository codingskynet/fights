use std::{thread, time::Duration};

use fights::{
    envs::BaseEnv,
    puoribor::{self, Action},
};
use rand::{rngs::StdRng, Rng, SeedableRng};

fn get_all_available_move(state: &puoribor::State, agent_id: usize) -> Vec<Action> {
    (0..9)
        .flat_map(|y| (0..9).map(move |x| (x, y)))
        .filter_map(|pos| {
            let action = puoribor::Action::new(0, pos);

            if puoribor::Env::step(state.clone(), agent_id, action.clone()).is_ok() {
                Some(action)
            } else {
                None
            }
        })
        .collect::<Vec<Action>>()
}

fn get_all_available_place_wall(state: &puoribor::State, agent_id: usize) -> Vec<Action> {
    if state.remaining_walls[agent_id] == 0 {
        return Vec::new();
    }

    // check place horizontal wall
    let horizontals = (0..9)
        .flat_map(|y| (0..10).map(move |x| (x, y)))
        .filter_map(|pos| {
            let action = puoribor::Action::new(1, pos);

            if puoribor::Env::step(state.clone(), agent_id, action.clone()).is_ok() {
                Some(action)
            } else {
                None
            }
        })
        .collect::<Vec<Action>>();

    // check place horizontal wall
    let verticals = (0..10)
        .flat_map(|y| (0..9).map(move |x| (x, y)))
        .filter_map(|pos| {
            let action = puoribor::Action::new(2, pos);

            if puoribor::Env::step(state.clone(), agent_id, action.clone()).is_ok() {
                Some(action)
            } else {
                None
            }
        })
        .collect::<Vec<Action>>();

    [horizontals, verticals].concat()
}

fn get_all_available_rotate_section(state: &puoribor::State, agent_id: usize) -> Vec<Action> {
    if state.remaining_walls[agent_id] < 2 {
        return Vec::new();
    }

    (0..6)
        .flat_map(|y| (0..6).map(move |x| (x, y)))
        .map(|pos| Action::new(3, pos))
        .collect::<Vec<Action>>()
}

fn get_all_available_action(state: &puoribor::State, agent_id: usize) -> Vec<Action> {
    [
        get_all_available_move(state, agent_id),
        get_all_available_place_wall(state, agent_id),
        get_all_available_rotate_section(state, agent_id),
    ]
    .concat()
}

#[test]
fn random_play() {
    let mut state = puoribor::Env::initialize_state();
    let mut rng = StdRng::seed_from_u64(0);

    let mut iter = 0;
    while state.is_win() == -1 {
        thread::sleep(Duration::from_millis(10));

        let agent_id = iter % 2;

        print!("{esc}[2J{esc}[1;1H", esc = 27 as char); // clear terminal
        println!("{}", state);

        let actions = get_all_available_action(&state, agent_id);
        let action = actions[rng.gen_range(0..actions.len())].clone();

        println!("Do: {:?}", action);

        match puoribor::Env::step(state.clone(), agent_id, action) {
            Ok(new_state) => {
                state = new_state;
            }
            Err(err_reason) => unreachable!("{}", err_reason),
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
