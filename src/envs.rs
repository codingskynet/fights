use core::fmt;

pub trait BaseState: fmt::Display {}

pub trait BaseEnv<S: BaseState, A> {
    fn env_id() -> (String, i32);

    fn initialize_state() -> Self;

    fn step(&mut self, state: S, agent_id: i32, action: A) -> S;
}

pub trait BaseAgent<S: BaseState, A> {
    fn env_id() -> (String, i32);

    fn new() -> Self;

    fn next(&self, state: S) -> A;
}
