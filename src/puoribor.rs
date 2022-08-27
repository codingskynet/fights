use std::fmt;

use ndarray::{Array, Array2};

use crate::{envs::*, utils::*, Err};

enum ActionType {
    Move = 0,                  // move to absolute position
    PlaceWallHorizontally = 1, // place horizontal wall with left position
    PlaceWallVertically = 2,   // place vertical wall with top position
    RotateSection = 3,         // rotate 4x4 section with top-left position
}

pub struct Action {
    action_type: ActionType,
    position: (usize, usize),
}

/*
 * Pouoribor's State
 * - players:
 *   - 0: the player 0's position(from top(4, 8))
 *   - 1: the player 1's position(from bottom(4, 8))
 * - board: four channel with size (9, 9). The (x, y) starts from top-left.
 *   - 0: one-hot encoded position of horizontal walls (size: (9, 10))
 *   - 1: one-hot encoded position of vertical walls (size: (10, 9))
 * - walls: the remaing walls on each player, (player 0's, player 1's)
 */
pub struct State {
    players: [(usize, usize); 2],
    board: [Array2<u8>; 2],
    remaning_walls: [usize; 2],
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let left_intersection_top = "┌";
        let middle_intersection_top = "┬";
        let right_intersection_top = "┐";

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
        let mut result = format!(
            "Remaing Walls\n - agent_0: {}\n - agent_1: {}\n",
            self.remaning_walls[0], self.remaning_walls[1]
        );

        result += left_intersection_top;

        for x in 0..9 {
            result += if self.board[0][[x, 0]] == 1 {
                horizontal_wall_bold
            } else {
                horizontal_wall
            };

            if x < 8 {
                result += middle_intersection_top;
            }
        }

        result = result + right_intersection_top + "\n";

        for y in 0..9 {
            result += if self.board[1][[0, y]] == 1 {
                vertical_wall_bold
            } else {
                vertical_wall
            };

            // display pawn and vertical wall
            for x in 0..9 {
                if self.players[0] == (x, y) {
                    result += " 0 ";
                } else if self.players[1] == (x, y) {
                    result += " 1 ";
                } else {
                    result += "   ";
                }

                if x < 8 {
                    if self.board[1][[x + 1, y]] == 1 {
                        result += vertical_wall_bold;
                    } else {
                        result += " ";
                    }
                }
            }

            result = result
                + if self.board[1][[9, y]] == 1 {
                    vertical_wall_bold
                } else {
                    vertical_wall
                }
                + "\n";

            // display horizontal wall
            if y < 8 {
                result += left_intersection;

                for x in 0..9 {
                    if self.board[0][[x, y + 1]] == 1 {
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

        for x in 0..9 {
            result = result
                + if self.board[0][[x, 9]] == 1 {
                    horizontal_wall_bold
                } else {
                    horizontal_wall
                };

            if x < 8 {
                result += middle_intersection_bottom;
            }
        }

        result = result + right_intersection_bottom;

        f.write_str(&result)
    }
}

impl State {
    pub fn new() -> Self {
        Self {
            players: [(4, 0), (4, 8)],
            board: [Array2::zeros([9, 10]), Array2::zeros([10, 9])],
            remaning_walls: [10, 10],
        }
    }
}

impl BaseState for State {}

pub struct Env {}

impl Env {
    // check if now and new is attached and there is no wall between them
    fn is_blocked_between(now: Position, new: Position, state: &State) -> bool {
        up(now) == new && state.board[0][now] == 1
            || down(now) == new && state.board[0][new] == 1
            || left(now) == new && state.board[1][now] == 1
            || right(now) == new && state.board[1][new] == 1
    }

    fn is_pawn_can_win(agent_id: usize, state: &State) -> bool {
        todo!()
    }
}

impl BaseEnv<State, Action> for Env {
    fn env_id() -> (String, i32) {
        todo!()
    }

    fn initialize_state() -> State {
        State::new()
    }

    fn step(&mut self, state: State, agent_id: usize, action: Action) -> Result<State, String> {
        match action.action_type {
            ActionType::Move => {
                let opposite = state.players[(agent_id + 1) % 2];
                let now = state.players[agent_id];
                let new = action.position;

                if new.0 >= 9 || new.1 >= 9 {
                    return Err!("Move: out of board");
                }

                if new == opposite {
                    return Err!("Move: cannot overlap the other pawn. How about jumping over it?");
                }

                let diff = diff_pos(now, new);
                if diff == 1 {
                    if Env::is_blocked_between(now, new, &state) {
                        return Err!("Move: the movement is blocked by wall.");
                    }
                } else if diff == 2 {
                    if Env::is_blocked_between(now, opposite, &state)
                        || Env::is_blocked_between(opposite, new, &state)
                    {
                        return Err!("Move: there is a wall between now pawn position, opposite pawn position, and new pawn position.");
                    }

                    // check straight jump over condition
                    if mid_pos(now, new) != opposite {
                        // check diagonal jump over condition
                        let l_or_r_opposite = left(opposite) == new || right(opposite) == new;
                        let u_or_d_opposite = up(opposite) == new || down(opposite) == new;

                        if !((up(now) == opposite
                            && state.board[0][opposite] == 1
                            && l_or_r_opposite)
                            || (down(now) == opposite
                                && state.board[0][down(opposite)] == 1
                                && l_or_r_opposite)
                            || (left(now) == opposite
                                && state.board[1][opposite] == 1
                                && u_or_d_opposite)
                            || (right(now) == opposite
                                && state.board[1][right(opposite)] == 1
                                && u_or_d_opposite))
                        {
                            return Err!("Move: cannot jump straightly or diagonally.");
                        }
                    }
                } else {
                    return Err!("Move: should move one block, not zero or bigger than one.");
                }

                let players = if agent_id == 0 {
                    [new, opposite]
                } else {
                    [opposite, new]
                };

                let state = State { players, ..state };

                if !Env::is_pawn_can_win((agent_id + 1) % 2, &state) {
                    return Err!("Move: this can make for the other player not to win.");
                }

                Ok(state)
            }
            ActionType::PlaceWallHorizontally => {
                if state.remaning_walls[agent_id] == 0 {
                    return Err!("PlaceWall: there is no remaing wall for the agent.");
                }

                let pos = action.position;

                if pos.0 >= 9 || pos.1 >= 10 {
                    return Err!("PlaceWall: out of board");
                }

                if state.board[0][pos] == 1 || state.board[0][right(pos)] == 1 {
                    return Err!("PlaceWall: there is already horizontal wall.");
                }

                if state.board[1][pos] == 1 && state.board[1][down(pos)] == 1 {
                    return Err!("PlaceWall: cannot install wall intersecting.");
                }

                let mut state = state;
                state.remaning_walls[agent_id] -= 1;
                state.board[0][pos] = 1;
                state.board[0][right(pos)] = 1;

                if !Env::is_pawn_can_win((agent_id + 1) % 2, &state) {
                    return Err!("Move: this can make for the other player not to win.");
                }

                Ok(state)
            }
            ActionType::PlaceWallVertically => {
                if state.remaning_walls[agent_id] == 0 {
                    return Err!("PlaceWall: there is no remaing wall for the agent.");
                }

                let pos = action.position;

                if pos.0 >= 10 || pos.1 >= 9 {
                    return Err!("PlaceWall: out of board");
                }

                if state.board[1][pos] == 1 || state.board[1][down(pos)] == 1 {
                    return Err!("PlaceWall: there is already vertical wall.");
                }

                if state.board[0][pos] == 1 && state.board[0][right(pos)] == 1 {
                    return Err!("PlaceWall: cannot install wall intersecting.");
                }

                let mut state = state;
                state.remaning_walls[agent_id] -= 1;
                state.board[1][pos] = 1;
                state.board[1][down(pos)] = 1;

                if !Env::is_pawn_can_win((agent_id + 1) % 2, &state) {
                    return Err!("Move: this can make for the other player not to win.");
                }

                Ok(state)
            }
            ActionType::RotateSection => {
                if state.remaning_walls[agent_id] <= 1 {
                    return Err!("RotationSection: there is no remainng wall for the agent.");
                }

                let pos = action.position;

                if pos.0 >= 6 || pos.1 >= 6 {
                    return Err!("RotationSection: out of board");
                }

                // horizontal -> vertial: make position to (x, y) => (4 - y, x)
                let mut new_vertial = Array2::zeros([5, 4]);

                for y in 0..=4 {
                    for x in 0..4 {
                        new_vertial[[4 - y, x]] = state.board[0][[pos.0 + x, pos.1 + y]];
                    }
                }

                // vertical -> horizontal: make positon to (x, y) => (3 - y, x)
                let mut new_horizontal = Array2::zeros([4, 5]);

                for y in 0..4 {
                    for x in 0..=4 {
                        new_horizontal[[3 - y, x]] = state.board[1][[pos.0 + x, pos.1 + y]];
                    }
                }

                let mut state = state;

                // apply them
                for y in 0..=4 {
                    for x in 0..4 {
                        state.board[0][[pos.0 + x, pos.1 + y]] = new_horizontal[[x, y]];
                    }
                }

                for y in 0..4 {
                    for x in 0..=4 {
                        state.board[1][[pos.0 + x, pos.1 + y]] = new_vertial[[x, y]];
                    }
                }

                if !Env::is_pawn_can_win((agent_id + 1) % 2, &state) {
                    return Err!("Move: this can make for the other player not to win.");
                }

                Ok(state)
            }
        }
    }
}
