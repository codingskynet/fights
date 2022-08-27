use std::fmt;

use ndarray::Array2;

use crate::envs::*;

pub struct Action {}

/*
 * Pouoribor's State
 * - board: four channel with size (9, 9). The (x, y) starts from bottom-left.
 *   - 0: one-hot encoded position of agent 0. (starts from top)
 *   ex) [0, 0, 0, 0, 1, 0, 0, 0, 0,
 *        0, 0, ...
 *        ...
 *        0, 0, 0, 0, 0, 0, 0, 0, 0,]
 *   - 1: one-hot encoded position of agent 1. (starts from bottom)
 *   ex) [0, 0, 0, 0, 0, 0, 0, 0, 0,
 *        0, 0, ...
 *        ...
 *        0, 0, 0, 0, 1, 0, 0, 0, 0,]
 *   - 2: one-hot encoded position of horizontal walls (size: (9, 10))
 *   - 3: one-hot encoded position of vertical walls (size: (10, 9))
 * - walls: the remaing walls on each player, (player 0's, player 1's)
 */
pub struct State {
    board: [Array2<u8>; 4],
    remaning_walls: (i32, i32),
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let table_top = "┌───┬───┬───┬───┬───┬───┬───┬───┬───┐";
        let vertical_wall = "│";
        let vertical_wall_bold = "┃";
        let horizontal_wall = "───";
        let horizontal_wall_bold = "━━━";
        let left_intersection = "├";
        let middle_intersection = "┼";
        let right_intersection = "┤";
        let left_intersection_bottom = "└";
        let middle_intersection_bottom = "┴";
        let right_intersection_bottom = "┘";
        let mut result = table_top.to_string() + "\n";
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
