use core::fmt;

pub trait BaseState: fmt::Display + Clone {}

pub trait BaseEnv<S: BaseState, A> {
    fn env_id() -> (String, i32);

    fn initialize_state() -> S;

    /// Step through the game
    ///
    /// On state S, append action (agent_id, action), then return new state if it possible, or return the failed reason
    fn step(state: S, agent_id: usize, action: A) -> Result<S, String>;
}

pub trait BaseAgent<S: BaseState, A> {
    fn env_id() -> (String, i32);

    fn new() -> Self;

    fn next(&self, state: S) -> A;
}
