use babylon::prelude::*;
#[macro_use]
extern crate lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref GAME: Mutex<Game> = Mutex::new(Game::new());
}

struct Game {
    scene: Scene,
    shape: Vec<Sphere>,
}

impl Game {
    fn new() -> Self {
        Game {
            scene: Scene::create_from_basic_engine("#renderCanvas"),
            shape: vec![],
        }
    }
}

#[no_mangle]
pub fn main() {
    babylon::js::log("Starting demo...");
    let mut game = GAME.lock().unwrap();
    for _ in 0..10 {
        let mut sphere = Sphere::new(&game.scene, babylon::js::random());
        sphere.set_position(Vector::new(
            babylon::js::random() - 0.5,
            babylon::js::random() - 0.5,
            babylon::js::random() - 0.5,
        ));
        game.shape.push(sphere);
    }
}
