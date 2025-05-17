#[derive(Clone, Copy)]
pub struct Config {
    pub col: usize,
    pub row: usize,
    pub mine: i32,
}

impl Config {
    pub fn new(c: usize, r: usize, mine: i32) -> Self {
        Self {
            col: c,
            row: r,
            mine: mine,
        }
    }

    pub fn Default() -> Self {
        Self {
            col: 11,
            row: 11,
            mine: 10,
        }
    }
}
