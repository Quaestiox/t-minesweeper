use std::{
    io::{self, Read, Write, stdout},
    process::exit,
    string,
    time::Instant,
};

use crate::terminal::{self, input::input, screen::Screen};

use super::cfg::Config;
use colored::Colorize;
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
    first: bool,
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
            first: true,
            screen: screen,
            config: cfg,
            world: world,
            board: board,
        }
    }

    pub fn init(&mut self) {
        //        self.screen.init();
    }

    pub fn one(&mut self) {
        self.generate_mine();
        self.generate_number();
        self.screen.clear_screen();
        stdout().flush();
    }

    pub fn draw(&self, all: bool) {
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
        h.push('Y');
        h.push('\n');
        let color_h = h.dark_red().to_string();
        self.screen.print(color_h).unwrap();

        for i in 0..cfg.row {
            let mut line = (i + 1).to_string().dark_blue().to_string();
            if i + 1 < 10 {
                line.push(' ');
            }
            line.push(' ');

            for j in 0..cfg.col {
                let c;
                if !board[i][j] && !all {
                    c = '·';
                } else {
                    c = world[i][j].render();
                }
                let color_c = render_color(c);
                line += color_c.as_str();
                line.push(' ');
                line.push(' ');
            }
            line.push('\n');
            self.screen.print(line).unwrap();
        }

        self.screen.print("X\n\n".dark_blue().to_string()).unwrap();
    }

    fn generate_mine(&mut self) {
        let Config { col, row, mine } = self.config;
        let mut rng = rand::thread_rng();
        let mut i = 0;
        while i < mine {
            let rd_col = rng.gen_range(0..col);
            let rd_row = rng.gen_range(0..row);

            if let Item::Space = self.world[rd_row][rd_col] {
                self.world[rd_row][rd_col] = Item::Mine;
                i += 1;
            } else {
                continue;
            }
        }
    }

    fn generate_mine_by_pos(&mut self, p_col: usize, p_row: usize) {
        let Config { col, row, mine } = self.config;
        let mut rng = rand::thread_rng();
        let mut rd_col;
        let mut rd_row;

        let mut i = 0;
        while i < mine {
            rd_col = rng.gen_range(0..col);
            rd_row = rng.gen_range(0..row);
            if rd_col != p_col || rd_row != p_row {
                if let Item::Space = self.world[rd_row][rd_col] {
                    self.world[rd_row][rd_col] = Item::Mine;
                    i += 1;
                } else {
                    continue;
                }
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
        let start = Instant::now();
        let cfg = self.config;
        loop {
            self.gprint("Input X: ");
            let mut input1 = String::new();
            io::stdin().read_line(&mut input1).unwrap();
            let x;
            match input1.trim().parse::<i32>() {
                Ok(num) => x = num - 1,
                Err(_) => {
                    self.gprint("please input a number\n");
                    continue;
                }
            };

            self.gprint("Input Y: ");

            let mut input2 = String::new();
            io::stdin().read_line(&mut input2).unwrap();
            let y;
            match input2.trim().parse::<i32>() {
                Ok(num) => y = num - 1,
                Err(_) => {
                    self.gprint("please input a number\n");
                    continue;
                }
            };

            if self.first {
                self.world = vec![vec![Item::Space; cfg.col]; cfg.row];
                self.generate_mine_by_pos(x as usize, y as usize);
                self.generate_number();
            }

            self.first = false;

            if x >= 0 && x < cfg.row as i32 && y >= 0 && y < cfg.col as i32 {
                match self.world[x as usize][y as usize] {
                    Item::Mine => {
                        self.screen.clear_screen();
                        self.screen.set_pos(0, 0);

                        self.draw(true);
                        self.screen.die().unwrap();
                        break;
                    }
                    Item::Number(num) => {
                        if self.board[x as usize][y as usize] {
                            self.gprint("already exploded\n");
                            continue;
                        }
                        self.board[x as usize][y as usize] = true;
                        self.spread(x, y);
                    }
                    _ => (),
                }
            } else {
                self.gprint("invalid number\n");
            }

            if self.judge() {
                let dura = start.elapsed();
                self.screen.clear_screen();
                self.screen.set_pos(0, 0);

                self.draw(true);
                self.screen.success(dura).unwrap();
                break;
            }
            self.screen.clear_screen().unwrap();
            self.draw(false);
        }
    }

    pub fn gprint(&self, s: &str) {
        self.screen.print(s.to_string()).unwrap();
        stdout().flush().unwrap();
    }

    pub fn spread(&mut self, i: i32, j: i32) {
        let direction: [i32; 3] = [-1, 0, 1];
        let Config { col, row, .. } = self.config;
        for x in 0..3 {
            for y in 0..3 {
                //                if x != 1 || y != 1 {
                let px = i as i32 + direction[x];
                let py = j as i32 + direction[y];

                if px >= 0 && px < row as i32 && py >= 0 && py < col as i32 {
                    match self.world[px as usize][py as usize] {
                        Item::Number(num) => {
                            if num == 0 {
                                if !self.board[px as usize][py as usize] {
                                    self.board[px as usize][py as usize] = true;
                                    self.spread(px, py);
                                }
                            } else {
                                if !self.board[px as usize][py as usize] {
                                    self.board[px as usize][py as usize] = true;
                                }
                            }
                        }
                        _ => (),
                    }
                }
                //               }
            }
        }
    }

    pub fn judge(&self) -> bool {
        let mut count = 0;
        let cfg = &self.config;
        let board = &self.board;
        for i in 0..cfg.row {
            for j in 0..cfg.col {
                if board[i][j] {
                    count += 1;
                }
            }
        }
        if (cfg.col * cfg.row) - count == cfg.mine as usize {
            return true;
        }
        false
    }
}

impl Item {
    pub fn render(&self) -> char {
        match self {
            Item::Space => '·',

            Item::Number(0) => ' ',
            Item::Number(num) => num.to_string().chars().next().unwrap(),
            Item::Mine => 'X',
        }
    }
}

fn render_color(c: char) -> String {
    match c {
        '1' => '1'.to_string().blue().to_string(),
        '2' => '2'.to_string().dark_green().to_string(),
        '3' => '3'.to_string().dark_red().to_string(),
        '4' => '4'.to_string().dark_blue().to_string(),
        '5' => '5'.to_string().to_string().dark_yellow().to_string(),
        '6' => '6'.to_string().dark_cyan().to_string(),
        '7' => '7'.to_string().black().to_string(),
        '8' => '8'.to_string().grey().to_string(),
        '·' => '·'.to_string().white().to_string(),
        'X' => 'X'.to_string().dark_magenta().to_string(),
        ' ' => ' '.to_string(),
        _ => "".to_string(),
    }
}
