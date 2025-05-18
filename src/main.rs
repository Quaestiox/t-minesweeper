mod game;
mod terminal;

use std::io::{Write, stdout};

use game::{cfg::Config, game::Game};
use terminal::{
    input::input,
    screen::{self, Screen},
};

fn main() {
    let screen = Screen::new();
    screen.init();
    loop {
        let cfg = screen.choose();

        let mut game = Game::new(cfg);

        game.one();
        game.draw(false);
        game.run();
        let str = input();
        screen.clear_screen().unwrap();
        screen.set_pos(0, 0).unwrap();
        stdout().flush().unwrap();

        if str == "c".to_string() {
            continue;
        } else if str == "q".to_string() {
            break;
        }
    }
}
