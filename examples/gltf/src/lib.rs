use babylon::prelude::*;
#[macro_use]
extern crate lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref GAME: Mutex<Game> = Mutex::new(Game::new());
}

struct Game {
    scene: Scene,
    models: Vec<GLTF>,
}

impl Game {
    fn new() -> Self {
        Game {
            scene: Scene::create_from_basic_engine("#renderCanvas"),
            models: vec![],
        }
    }
}

#[no_mangle]
pub fn main() {
    let mut game = GAME.lock().unwrap();
    let mut gltf = GLTF::new(&game.scene, "BoomBox.gltf");
    gltf.set_scaling(Vector::new(50.0, 50.0, 50.0));
    game.models.push(gltf);
}
