mod game;
mod terminal;

use game::{cfg::Config, game::Game};
use terminal::screen::{self, Screen};

fn main() {
    let cfg = Config::Default();
    let mut game = Game::new(cfg);
    game.init();
    game.draw();
    game.run();
}
