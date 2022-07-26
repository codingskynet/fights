"""
Pouribor, a variant of the classical `Quoridor <https://en.wikipedia.org/wiki/Quoridor>`_ game.

Coordinates are specified in the form of `(x, y)`, where `(0, 0)` is the bottom left corner.
All coordinates and directions are absolute and does not change between agents.

Directions
    - Top: `+y`
    - Right: `+x`
    - Bottom: `-y`
    - Left: `-x`
"""

from __future__ import annotations

from dataclasses import dataclass
from typing import Callable
from typing_extensions import TypeAlias
import numpy as np
from numpy.typing import NDArray
from scipy.signal import convolve2d


Action: TypeAlias = NDArray[np.int_]
"""
Action type. Encoded as a NumPy array of shape ``(4,)``, in the form of [ `agent_id`, `action_type`, `coordinate_x`, `coordinate_y` ]
.

`agent_id`
    - agent id of action (0 or 1)

`action_type`
    - 0 (move piece)
    - 1 (place wall horizontally)
    - 2 (place wall vertically)
    - 3 (rotate section)

`coordinate_x`, `coordinate_y`
    - position to move the piece to
    - top or left position to place the wall
    - top left position of the section to rotate
"""


@dataclass
class PouriborState:
    """
    PouriborState class.

    `board`
        - Array of shape ``(C, W, H)``, where C is channel index and W, H is board width, height.
        - C = 0: one-hot encoded position of agent 0. (starts from top)
        - C = 1: one-hot encoded position of agent 1. (starts from bottom)
        - C = 2: one-hot encoded positions of horizontal walls.
        - C = 3: one-hot encoded positions of vertical walls.

    `walls_remaining`
        - Array of shape ``(2,)``, in the form of [ `agent0_remaining_walls`, `agent1_remaining_walls` ]

    `done`
        - Boolean value indicating whether the game is done.
    """

    board: NDArray[np.int_]
    walls_remaining: NDArray[np.int_]
    done: bool = False


@dataclass
class Result:
    state: PouriborState
    reward_fn: Callable
    done: bool


class PouriborEnv:
    board_size: int = 9
    """
    Size (width and height) of the board.
    """

    max_walls: int = 20
    """
    Maximum allowed walls per agent.
    """

    def step(self, state: PouriborState, action: Action) -> PouriborState:
        """
        Step through the game, calculating the next state given the current state and action to take.

        Args:
            target: the object of which the state should be restored.
            state: a dictionary generated by `to_state_dict` with the desired new state for `target`.

        Returns:
            A copy of the object with the restored state.
        """

        return PouriborState(
            board=state.board.at[action[0], action[1]].set(1),
            won=self._check_wins(state),
        )

    def _check_wins(self, state: PouriborState) -> bool:
        kernels = [
            np.eye(self.win_condition),
            np.fliplr(np.eye(self.win_condition)),
            np.ones((self.win_condition, 1)),
            np.ones((1, self.win_condition)),
        ]
        for kernel in kernels:
            convolved = convolve2d(state.board, kernel)
            if np.any(convolved >= self.win_condition):
                return True
        return False
