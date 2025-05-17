use std::{
    io::{self, Read, Write, stdout},
    process::exit,
    string,
};

use crate::terminal::{self, screen::Screen};

use super::cfg::Config;
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers, read},
    style::Stylize,
};
use rand::Rng;

#[derive(Clone, Copy)]
enum Item {
    Space,
    Mine,
    Number(i8),
}

pub struct Game {
    shut_down: bool,
    screen: Screen,
    config: Config,
    world: Vec<Vec<Item>>,
    board: Vec<Vec<bool>>,
}

impl Game {
    pub fn new(cfg: Config) -> Self {
        // init world
        let world: Vec<Vec<Item>> = vec![vec![Item::Space; cfg.col]; cfg.row];
        // init screen
        let screen = Screen::new();
        //init board
        let board: Vec<Vec<bool>> = vec![vec![false; cfg.col]; cfg.row];

        Self {
            shut_down: false,
            screen: screen,
            config: cfg,
            world: world,
            board: board,
        }
    }

    pub fn init(&mut self) {
        self.screen.init();

        self.generate_mine();
        self.generate_number();
    }

    pub fn draw(&self) {
        let cfg = &self.config;
        let world = &self.world;
        let board = &self.board;

        let mut h: String = if cfg.col < 10 {
            "   ".to_string()
                + (1..=cfg.col)
                    .map(|i| i.to_string())
                    .collect::<Vec<_>>()
                    .join("  ")
                    .as_str()
        } else {
            "   ".to_string()
                + (1..=10)
                    .map(|i| i.to_string())
                    .collect::<Vec<_>>()
                    .join("  ")
                    .as_str()
                + " "
                + (11..=cfg.col)
                    .map(|i| i.to_string())
                    .collect::<Vec<_>>()
                    .join(" ")
                    .as_str()
        };
        h.push(' ');
        h.push('y');
        h.push('\n');

        self.screen.print(h).unwrap();

        for i in 0..cfg.row {
            let mut line = (i + 1).to_string();
            if i + 1 < 10 {
                line.push(' ');
            }
            line.push(' ');

            for j in 0..cfg.col {
                let c;
                if !board[i][j] {
                    c = '·';
                } else {
                    c = world[i][j].render();
                }
                line.push(c);
                line.push(' ');
                line.push(' ');
            }
            line.push('\n');
            self.screen.print(line).unwrap();
        }

        self.screen.print("x\n\n".to_string()).unwrap();
    }

    fn generate_mine(&mut self) {
        let Config { col, row, mine } = self.config;
        let mut rng = rand::thread_rng();
        for _ in 0..mine {
            let rd_col = rng.gen_range(0..col);
            let rd_row = rng.gen_range(0..row);
            if let Item::Space = self.world[rd_row][rd_col] {
                self.world[rd_row][rd_col] = Item::Mine;
            }
        }
    }

    fn generate_number(&mut self) {
        let direction: [i8; 3] = [-1, 0, 1];
        let Config { col, row, .. } = self.config;
        for i in 0..row {
            for j in 0..col {
                let mut count = 0;
                for x in 0..3 {
                    for y in 0..3 {
                        if x != 1 || y != 1 {
                            let px = i as i8 + direction[x];
                            let py = j as i8 + direction[y];

                            if px >= 0 && px < row as i8 && py >= 0 && py < col as i8 {
                                if let Item::Mine = self.world[px as usize][py as usize] {
                                    count += 1;
                                }
                            }
                        }
                    }
                }
                if let Item::Space = self.world[i][j] {
                    self.world[i][j] = Item::Number(count);
                }
            }
        }
    }

    pub fn run(&mut self) {
        let cfg = self.config;
        loop {
            self.screen.print("input x: ".to_string()).unwrap();
            stdout().flush().unwrap();
            let mut input1 = String::new();
            io::stdin().read_line(&mut input1).unwrap();
            let x;
            match input1.trim().parse::<i32>() {
                Ok(num) => x = num,
                Err(_) => {
                    self.screen
                        .print("please input a number\n".to_string())
                        .unwrap();
                    stdout().flush().unwrap();
                    continue;
                }
            };

            self.screen.print("input y: ".to_string()).unwrap();
            stdout().flush().unwrap();
            let mut input2 = String::new();
            io::stdin().read_line(&mut input2).unwrap();
            let y;
            match input1.trim().parse::<i32>() {
                Ok(num) => y = num,
                Err(_) => {
                    self.screen
                        .print("please input a number\n".to_string())
                        .unwrap();
                    stdout().flush().unwrap();
                    continue;
                }
            };

            if x > 0 && x < cfg.row as i32 && y > 0 && y < cfg.col as i32 {
                if self.board[x as usize][y as usize] {
                    self.gprint("already exploded");
                    continue;
                }
                self.board[x as usize][y as usize] = true;
            } else {
                self.screen
                    .print("please input a number\n".to_string())
                    .unwrap();
                stdout().flush().unwrap();
                continue;
            }
            self.screen.clear_screen().unwrap();
            self.draw();
        }
    }

    pub fn gprint(&self, s: &str) {
        self.screen.print(s.to_string()).unwrap();
        stdout().flush().unwrap();
    }

    pub fn input(&mut self) -> String {
        loop {
            if let Ok(event) = read() {
                match event {
                    Event::Key(key_event) => match key_event {
                        KeyEvent {
                            code,
                            modifiers,
                            kind,
                            state,
                        } => match code {
                            KeyCode::Char(c) => match c {
                                'q' if modifiers == KeyModifiers::CONTROL => {
                                    exit(0);
                                    self.shut_down = true;

                                    return "quit".to_string();
                                }
                                //                                '1'..='9' => return c,
                                _ => (),
                            },
                            KeyCode::Enter => return "enter".to_string(),

                            _ => (),
                        },
                    },
                    _ => (),
                }
            }
        }
    }

    pub fn handle(&mut self, c: char) {}
}

impl Item {
    pub fn render(&self) -> char {
        match self {
            Item::Space => '·',
            Item::Number(num) => num.to_string().chars().next().unwrap(),
            Item::Mine => 'X',
        }
    }
}
