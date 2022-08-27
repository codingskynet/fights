use std::fmt;

use ndarray::Array2;

use crate::envs::*;

pub struct Action {}

/*
 * Pouoribor's State
 * - board: four channel with size (9, 9). The (x, y) starts from top-left.
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

        for y in 0..9 {
            result += vertical_wall;

            // display pawn and vertical wall
            for x in 0..9 {
                if self.board[0][[x, y]] == 1 {
                    result += " 0 ";
                } else if self.board[1][[x, y]] == 1 {
                    result += " 1 ";
                } else {
                    result += "   ";
                }

                if x < 8 {
                    if self.board[3][[x, y]] == 1 {
                        result += vertical_wall_bold;
                    } else {
                        result += " ";
                    }
                }
            }

            result = result + vertical_wall + "\n";

            // display horizontal wall
            if y < 8 {
                result += left_intersection;

                for x in 0..9 {
                    if self.board[2][[x, y]] == 1 {
                        result += horizontal_wall_bold;
                    } else {
                        result += "   ";
                    }

                    if x < 8 {
                        result += middle_intersection
                    }
                }

                result += right_intersection;
                result += "\n";
            }
        }

        // display the end bottom line
        result = result + left_intersection_bottom;

        for _ in 0..8 {
            result = result + horizontal_wall + middle_intersection_bottom;
        }

        result = result + horizontal_wall + right_intersection_bottom;

        f.write_str(&result)
    }
}

impl State {
    pub fn new() -> Self {
        let mut player0 = Array2::zeros([9, 9]);
        player0[[4, 0]] = 1;

        let mut player1 = Array2::zeros([9, 9]);
        player1[[4, 8]] = 1;

        Self {
            board: [
                player0,
                player1,
                Array2::zeros([9, 9]),
                Array2::zeros([9, 9]),
            ],
            remaning_walls: (10, 10),
        }
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
