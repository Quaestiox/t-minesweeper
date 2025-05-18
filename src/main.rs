mod game;
mod terminal;

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
        game.draw();
        game.run();
        let str = input();
        if str == "enter".to_string() {
            continue;
        } else if str == "q".to_string() {
            break;
        }
    }
}
