#[macro_export]
macro_rules! Err {
    ($err_str:literal) => {
        Err($err_str.to_string())
    };
}

pub type Position = (u8, u8);
pub type PositionIdx = (usize, usize);

#[inline]
pub fn i(x: Position) -> PositionIdx {
    (x.0 as usize, x.1 as usize)
}

// calculate ||x - y|| that the norm is L_0
#[inline]
pub fn diff_pos(x: Position, y: Position) -> usize {
    (x.0.abs_diff(y.0) + x.1.abs_diff(y.1)) as usize
}

// calculate if the b is between a and c
#[inline]
pub fn is_mid_pos(a: Position, b: Position, c: Position) -> bool {
    if a.0 == c.0 || a.1 == c.1 {
        b == ((a.0 + c.0) / 2, (a.1 + c.1) / 2)
    } else {
        false
    }
}

// move the position to up(y: -1)
#[inline]
pub fn up(p: Position) -> Position {
    (p.0, p.1 - 1)
}

// move the position to down(y: +1)
#[inline]
pub fn down(p: Position) -> Position {
    (p.0, p.1 + 1)
}

// move the position to left(x: -1)
#[inline]
pub fn left(p: Position) -> Position {
    (p.0 - 1, p.1)
}

// move the position to right(x: +1)
#[inline]
pub fn right(p: Position) -> Position {
    (p.0 + 1, p.1)
}
