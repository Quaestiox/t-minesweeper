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

    pub fn easy() -> Self {
        Self {
            col: 8,
            row: 8,
            mine: 10,
        }
    }
    pub fn normal() -> Self {
        Self {
            col: 16,
            row: 16,
            mine: 40,
        }
    }
    pub fn hard() -> Self {
        Self {
            col: 16,
            row: 30,
            mine: 99,
        }
    }
    pub fn default() -> Self {
        Self {
            col: 11,
            row: 11,
            mine: 2,
        }
    }
}
