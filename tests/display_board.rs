use fights::puoribor;

#[test]
fn display_board() {
    let init_state = puoribor::State::new();

    println!("{}", init_state);
}
