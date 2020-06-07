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
    let game = GAME.lock().unwrap();
    game.scene.add_keyboard_observable(|event_type, key_code| {
        if event_type == KEYUP {
            babylon::js::log(&format!("{} {}", event_type, key_code));
            let mut game = GAME.lock().unwrap();
            let mut sphere = Sphere::new(&game.scene, babylon::js::random());
            sphere.set_position(
                babylon::js::random() - 0.5,
                babylon::js::random() - 0.5,
                babylon::js::random() - 0.5,
            );
            game.shape.push(sphere);
        }
    })
}
