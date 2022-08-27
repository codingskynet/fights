<div align="center">

# rust-fights

The alternative environment for running Puoribor on Rust

</div>

---

This version(`rust/main`) can be slightly different from the original one(`main`) with respect to algorithm or logic. But, this is for some optimizations or my preference and the execution and output file(like serialized data) should be same to the original one.

## English

### Game Rule

This game is a variant of [Quoridor](https://en.wikipedia.org/wiki/Quoridor). The main board is 9x9 and each player has 10 walls whose size is 2.

New rule:
- The player can rotate 4x4 local board 90° clockwisely by consuming 2 walls.
  - The rotation only affects the walls on local board, not pawns' position.
  - But, this action cannot be allowed if the other pawn cannot move to the winning area(the opposite side of the board) by obeying the original game rule.
  - If there is a wall crossing the side of the local board, the wall of size 2 splits into each walls of size 1.
  - You cannot rotate local board that some part is outside of the main board.


Basic Rule(according to Quoridor):
- On your turn, you can move your pawn, install wall, or rotate the local board.
  - You cannot drop out your turn.
- The pawn can move to 4 directions(up, down, left, right).
  - Your pawn can jump over that pawn if there is the other pawn toward chosen direction. If there is a wall behind the other pawn which is in front of your pawn, you can jump diagonally.
  - Your pawn cannot move if there is a wall toward chosen direction.
- You can install your wall anywhere.
  - You cannot install your wall if the other pawn cannot move to the winning area(the opposite side of the board) after installing.
  - You cannot install your wall overlaping or crossing already installed wall(be like + shape).
  - You cannot install your wall that only the wall of size 1 is inside on the board.
- The player wins if their pawn arrived at the opposite side of the board and the game is done.

## Korean

### 게임 규칙

쿼리도([Quoridor](https://en.wikipedia.org/wiki/Quoridor))의 변형 게임이다. 9x9 크기의 보드에서 각 플레이어별로 2칸짜리 장애물을 10개씩 가지고 게임을 진행한다.

다음과 같은 규칙이 추가됐다.

- 가지고 있는 장애물 2개를 소모하여, 원하는 위치의 4x4 크기의 보드를 시계방향으로 90도 회전시킬 수 있다.
  - 4x4 크기의 보드 위에 있는 모든 장애물의 위치가 바뀌지만, 말은 회전에 영향을 받지 않는다.
  - 단, 쿼리도의 기본 규칙을 따라 상대가 빠져나갈 수 없는 경로를 만드는 경우에는 둘 수 없는 행위이다.
  - 4x4 크기의 보드를 돌리는 중에 2칸짜리 장애물이 걸쳐져 있다면, 부서지고 각각 길이가 1인 장애물로 분해가 된다.
  - 4x4 크기의 보드는 메인 보드 안에 있는 보드여야 한다. 즉, 일부가 메인 보드 밖에 걸쳐져 있으면 안 된다.

기본 규칙(쿼리도의 룰을 따른다):
- 자신의 차례에서 말을 움직이거나, 장애물을 설치하거나, 보드를 회전시킬 수 있다.
  - 자신의 차례를 포기할 수 없다.
- 말은 상, 하, 좌, 우 네 방향으로 한 칸씩 움직일 수 있다.
  - 이동하려는 방향에 상대방 말이 있다면 뛰어 넘을 수 있다. 만약 상대 말 뒤에 벽이 있다면 대각선으로 뛰어 넘을 수 있다.
  - 이동하려는 방향에 장애물이 있다면 움직일 수 없다.
- 장애물을 원하는 곳에 설치할 수 있다.
  - 상대가 빠져나갈 수 없는 경로를 만드는 경우에는 설치할 수 없다.
  - 장애물을 겹치거나, 서로 가로지르게(즉, +가 되도록) 설치할 수 없다.
  - 보드 밖을 나가도록(즉, 보드에 한 칸만 걸쳐지도록) 장애물을 설치할 수 없다.
- 어느 플레이어의 말이 반대편 보드의 끝에 도달하면 승리하고 게임이 종료된다.
