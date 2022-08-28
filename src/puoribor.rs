use std::{collections::VecDeque, fmt, hash::Hash};

use ndarray::Array2;

use crate::{envs::*, utils::*, Err};

#[derive(Debug, Clone, PartialEq)]
pub enum ActionType {
    Move = 0,                  // move to absolute position
    PlaceWallHorizontally = 1, // place horizontal wall with left position
    PlaceWallVertically = 2,   // place vertical wall with top position
    RotateSection = 3,         // rotate 4x4 section with top-left position
}

impl From<usize> for ActionType {
    fn from(id: usize) -> Self {
        match id {
            0 => ActionType::Move,
            1 => ActionType::PlaceWallHorizontally,
            2 => ActionType::PlaceWallVertically,
            3 => ActionType::RotateSection,
            _ => panic!("Cannot parse id: {}", id),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Action {
    action_type: ActionType,
    position: Position,
}

impl Action {
    pub fn new(action_type: usize, position: Position) -> Self {
        Self {
            action_type: ActionType::from(action_type),
            position,
        }
    }
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
#[derive(Debug, Clone, Hash, PartialEq)]
pub struct State {
    pub players: [(usize, usize); 2],
    pub board: [Array2<u8>; 2],
    pub remaining_walls: [usize; 2],
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.display_with(None))
    }
}

impl State {
    pub fn new() -> Self {
        Self {
            players: [(4, 0), (4, 8)],
            board: [Array2::zeros([9, 10]), Array2::zeros([10, 9])],
            remaining_walls: [10, 10],
        }
    }

    pub fn is_win(&self) -> isize {
        if self.players[0].1 == 8 {
            0
        } else if self.players[1].1 == 0 {
            1
        } else {
            -1
        }
    }

    pub fn display_with(&self, marker_board: Option<(&str, Array2<u8>)>) -> String {
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
            "Remaining Walls\n - agent_0: {}\n - agent_1: {}\n",
            self.remaining_walls[0], self.remaining_walls[1]
        );

        // display x coordinate
        result += "    ";
        for x in 0..9 {
            result += &format!(" {}  ", x);
        }
        result += " \n   ";

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
            result += &format!(" {} ", y);
            result += if self.board[1][[0, y]] == 1 {
                vertical_wall_bold
            } else {
                vertical_wall
            };

            // display pawn and vertical wall
            for x in 0..9 {
                result = result + " " + if self.players[0] == (x, y) {
                    "0"
                } else if self.players[1] == (x, y) {
                    "1"
                } else if let Some((marker, ref board)) = marker_board && board[[x, y]] == 1 {
                   marker
                } else {
                   " "
                }
                + " ";

                if x < 8 {
                    result += if self.board[1][[x + 1, y]] == 1 {
                        vertical_wall_bold
                    } else {
                        " "
                    };
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
                result += "   ";
                result += left_intersection;

                for x in 0..9 {
                    result += if self.board[0][[x, y + 1]] == 1 {
                        horizontal_wall_bold
                    } else {
                        "   "
                    };

                    if x < 8 {
                        result += middle_intersection
                    }
                }

                result += right_intersection;
                result += "\n";
            }
        }

        // display the end bottom line
        result = result + "   " + left_intersection_bottom;

        for x in 0..9 {
            result += if self.board[0][[x, 9]] == 1 {
                horizontal_wall_bold
            } else {
                horizontal_wall
            };

            if x < 8 {
                result += middle_intersection_bottom;
            }
        }

        result = result + right_intersection_bottom;

        result
    }
}

impl BaseState for State {}

pub struct Env {}

impl Env {
    // check if now and new is attached and there is no wall between them
    fn is_blocked_between(now: Position, new: Position, state: &State) -> bool {
        (now.1 > 0 && up(now) == new && state.board[0][now] == 1)
            || (now.1 < 8 && down(now) == new && state.board[0][new] == 1)
            || (now.0 > 0 && left(now) == new && state.board[1][now] == 1)
            || (now.0 < 8 && right(now) == new && state.board[1][new] == 1)
    }

    fn is_pawn_can_win(agent_id: usize, state: &State) -> bool {
        let mut queue = VecDeque::new();

        let win_y = if agent_id == 0 { 8 } else { 0 };
        queue.push_back(state.players[agent_id]);

        let mut visited = Array2::zeros([9, 9]);

        while let Some(pos) = queue.pop_front() {
            if visited[pos] == 1 {
                continue;
            }

            visited[pos] = 1;

            if pos.1 == win_y {
                return true;
            }

            if pos.1 > 0 && state.board[0][pos] != 1 {
                queue.push_back(up(pos));
            }

            if pos.1 < 8 && state.board[0][down(pos)] != 1 {
                queue.push_back(down(pos));
            }

            if pos.0 > 0 && state.board[1][pos] != 1 {
                queue.push_back(left(pos));
            }

            if pos.0 < 8 && state.board[1][right(pos)] != 1 {
                queue.push_back(right(pos));
            }
        }

        false
    }
}

impl BaseEnv<State, Action> for Env {
    fn env_id() -> (String, i32) {
        todo!()
    }

    fn initialize_state() -> State {
        State::new()
    }

    fn step(state: State, agent_id: usize, action: Action) -> Result<State, String> {
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
                    if !is_mid_pos(now, opposite, new) {
                        // check diagonal jump over condition

                        if !((now.1 > 0
                            && up(now) == opposite
                            && (state.board[0][opposite] == 1 || opposite.1 == 0)
                            && ((now.0 > 0 && left(opposite) == new)
                                || (now.0 < 8 && right(opposite) == new)))
                            || (now.1 < 8
                                && down(now) == opposite
                                && (state.board[0][down(opposite)] == 1 || opposite.1 == 8)
                                && ((now.0 > 0 && left(opposite) == new)
                                    || (now.0 < 8 && right(opposite) == new)))
                            || (now.0 > 0
                                && left(now) == opposite
                                && (state.board[1][opposite] == 1 || opposite.0 == 0)
                                && ((now.1 > 0 && up(opposite) == new)
                                    || (now.1 < 8 && down(opposite) == new)))
                            || (now.0 < 8
                                && right(now) == opposite
                                && (state.board[1][right(opposite)] == 1 || opposite.0 == 8)
                                && ((now.1 > 0 && up(opposite) == new)
                                    || (now.1 < 8 && down(opposite) == new))))
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

                Ok(state)
            }
            ActionType::PlaceWallHorizontally => {
                if state.remaining_walls[agent_id] == 0 {
                    return Err!("PlaceWallHorizontally: there is no remaing wall for the agent.");
                }

                let pos = action.position;

                if pos.0 >= 8 || pos.1 == 0 || pos.1 >= 9 {
                    return Err!("PlaceWallHorizontally: out of board");
                }

                if state.board[0][pos] == 1 || state.board[0][right(pos)] == 1 {
                    return Err!("PlaceWallHorizontally: there is already horizontal wall.");
                }

                if state.board[1][pos] == 1 && pos.1 < 8 && state.board[1][down(pos)] == 1 {
                    return Err!(
                        "PlaceWallHorizontally: cannot install horizontal wall intersecting."
                    );
                }

                let mut state = state;
                state.remaining_walls[agent_id] -= 1;
                state.board[0][pos] = 1;
                state.board[0][right(pos)] = 1;

                if !(Env::is_pawn_can_win(agent_id, &state)
                    && Env::is_pawn_can_win((agent_id + 1) % 2, &state))
                {
                    return Err!(
                        "PlaceWallHorizontally: this can make for the other player not to win."
                    );
                }

                Ok(state)
            }
            ActionType::PlaceWallVertically => {
                if state.remaining_walls[agent_id] == 0 {
                    return Err!("PlaceWallVertically: there is no remaing wall for the agent.");
                }

                let pos = action.position;

                if pos.0 == 0 || pos.0 >= 9 || pos.1 >= 8 {
                    return Err!("PlaceWallVertically: out of board");
                }

                if state.board[1][pos] == 1 || state.board[1][down(pos)] == 1 {
                    return Err!("PlaceWallVertically: there is already vertical wall.");
                }

                if state.board[0][pos] == 1 && pos.0 < 8 && state.board[0][right(pos)] == 1 {
                    return Err!("PlaceWallVertically: cannot install vertical wall intersecting.");
                }

                let mut state = state;
                state.remaining_walls[agent_id] -= 1;
                state.board[1][pos] = 1;
                state.board[1][down(pos)] = 1;

                if !(Env::is_pawn_can_win(agent_id, &state)
                    && Env::is_pawn_can_win((agent_id + 1) % 2, &state))
                {
                    return Err!(
                        "PlaceWallVertically: this can make for the other player not to win."
                    );
                }

                Ok(state)
            }
            ActionType::RotateSection => {
                if state.remaining_walls[agent_id] <= 1 {
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

                // remove the edge walls
                for i in 0..9 {
                    state.board[0][[i, 0]] = 0;
                    state.board[0][[i, 9]] = 0;
                    state.board[1][[0, i]] = 0;
                    state.board[1][[9, i]] = 0;
                }

                if !(Env::is_pawn_can_win(agent_id, &state)
                    && Env::is_pawn_can_win((agent_id + 1) % 2, &state))
                {
                    return Err!("RotationSection: this can make for the other player not to win.");
                }

                Ok(state)
            }
        }
    }
}
