use babylon::*;
#[macro_use]
extern crate lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref GAME: Mutex<Game> = Mutex::new(Game::default());
}

struct Game {
    scene: Scene,
    shape: Vec<Sphere>,
}

impl Default for Game {
    fn default() -> Self {
        babylon::js::log("hey2");
        Game {
            scene: Scene::create_from_basic_engine("#renderCanvas"),
            shape: vec![],
        }
    }
}

#[no_mangle]
pub fn main() {
    let mut game = GAME.lock().unwrap();
    for _ in 0..10 {
        let mut sphere = Sphere::create_sphere(&game.scene, babylon::js::random());
        sphere.set_position(
            babylon::js::random(),
            babylon::js::random(),
            -babylon::js::random(),
        );
        game.shape.push(sphere);
    }
}
