use std::fmt;

use ndarray::Array2;

use crate::envs::*;

pub struct Action {}

/*
 * Pouoribor's State has four channel. The (x, y) starts from bottom-left.
 * - 0: one-hot encoded position of agent 0. (starts from top)
 * - 1: one-hot encoded position of agent 1. (starts from bottom)
 * - 2: one-hot encoded position of horizontal walls
 * - 3: one-hot encoded position of vertical walls
 */
pub struct State {
    board: [Array2<u8>; 4],
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl BaseState for State {}

pub struct Env {}

impl BaseEnv<State, Action> for Env {
    fn env_id() -> (String, i32) {
        todo!()
    }

    fn initialize_state() -> Self {
        todo!()
    }

    fn step(&mut self, state: State, agent_id: i32, action: Action) -> State {
        todo!()
    }
}

pub struct Agent {}

impl BaseAgent<State, Action> for Agent {
    fn env_id() -> (String, i32) {
        todo!()
    }

    fn new() -> Self {
        todo!()
    }

    fn next(&self, state: State) -> Action {
        todo!()
    }
}
