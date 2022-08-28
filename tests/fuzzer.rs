use std::{panic, thread, time::Duration};

use fights::{
    envs::BaseEnv,
    puoribor::{self, Action},
};
use rand::{rngs::StdRng, Rng, SeedableRng};
use rayon::prelude::*;

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
        .filter_map(|pos| {
            let action = puoribor::Action::new(3, pos);

            if puoribor::Env::step(state.clone(), agent_id, action.clone()).is_ok() {
                Some(action)
            } else {
                None
            }
        })
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

fn random_play_with_seed(seed: u64, delay: u64, slient: bool) {
    let mut state = puoribor::Env::initialize_state();
    let mut rng = StdRng::seed_from_u64(seed);

    let mut iter = 0;
    while state.is_win() == -1 {
        thread::sleep(Duration::from_millis(delay));

        let agent_id = iter % 2;

        let actions = get_all_available_action(&state, agent_id);
        let action = actions[rng.gen_range(0..actions.len())].clone();

        if !slient {
            print!("{esc}[2J{esc}[1;1H", esc = 27 as char); // clear terminal
            println!("seed: {}", seed);
            println!("{}", state);
            println!("Player {} will do: {:?}", agent_id, action);
        }

        match puoribor::Env::step(state.clone(), agent_id, action) {
            Ok(new_state) => {
                state = new_state;
            }
            Err(err_reason) => unreachable!("{}", err_reason),
        }

        iter += 1;
    }

    if !slient {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char); // clear terminal
        println!("{}", state);
        println!("iters: {}", iter);
        if state.is_win() == 0 {
            println!("The player 0 is won!");
        } else {
            println!("The player 1 is won!");
        }
    }
}

#[test]
fn random_play() {
    random_play_with_seed(0, 0, false);
}

#[test]
fn fuzzer() {
    let seeds = (0..1_000_000).collect::<Vec<u64>>();
    let error_seeds = seeds
        .par_iter()
        .map(|i| {
            (
                i,
                panic::catch_unwind(|| random_play_with_seed(*i, 0, true)),
            )
        })
        .filter_map(|(i, result)| if result.is_ok() { None } else { Some(i) })
        .collect::<Vec<_>>();

    assert_eq!(error_seeds, Vec::<&u64>::new());
}
