#[macro_export]
macro_rules! Err {
    ($err_str:literal) => {
        Err($err_str.to_string())
    };
}

type Position = (usize, usize);

// calculate |x - y| that the norm is L_0
pub fn diff_pos(x: Position, y: Position) -> usize {
    x.0.abs_diff(y.0) + x.1.abs_diff(y.1)
}

// calculate the middle position: (x + y) / 2
pub fn mid_pos(x: Position, y: Position) -> Position {
    ((x.0 + y.0) / 2, (x.1 + y.1) / 2)
}

// move the position to up(y: -1)
pub fn up(p: Position) -> Position {
    (p.0, p.1 - 1)
}

// move the position to down(y: +1)
pub fn down(p: Position) -> Position {
    (p.0, p.1 + 1)
}

// move the position to left(x: -1)
pub fn left(p: Position) -> Position {
    (p.0 - 1, p.1)
}

// move the position to right(x: +1)
pub fn right(p: Position) -> Position {
    (p.0 + 1, p.1)
}
