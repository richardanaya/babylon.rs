use babylon::*;

#[macro_use]
extern crate lazy_static;

use std::sync::Mutex;

lazy_static! {
    static ref GAME: Mutex<Game> = Mutex::new(Game::new());
}

struct Game {
    scene: Scene,
}

impl Game {
    fn new() -> Game {
        Game {
            scene: Scene::create_from_basic_engine("#renderCanvas"),
        }
    }

    fn init(&mut self) {}
}

#[no_mangle]
pub fn main() {
    let mut game = GAME.lock().unwrap();
    game.init();
}
