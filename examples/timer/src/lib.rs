use babylon::prelude::*;
#[macro_use]
extern crate lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref GAME: Mutex<Game> = Mutex::new(Game::new());
}

struct Game {
    time: f64,
    scene: Scene,
    shape: Vec<Sphere>,
}

impl Game {
    fn new() -> Self {
        Game {
            scene: Scene::create_from_basic_engine("#renderCanvas"),
            shape: vec![],
            time: 0.0,
        }
    }
}

#[no_mangle]
pub fn main() {
    let game = GAME.lock().unwrap();
    game.scene.add_before_render_observable(|| {
        let mut game = GAME.lock().unwrap();
        let delta_time = game.scene.get_delta_time();
        game.time += delta_time;
        if game.time > 1000.0 {
            game.time -= 1000.0;
            // add sphere every second
            let mut sphere = Sphere::new(&game.scene, babylon::js::random());
            sphere.set_position(Vector::new(
                babylon::js::random() - 0.5,
                babylon::js::random() - 0.5,
                babylon::js::random() - 0.5,
            ));
            game.shape.push(sphere);
        }
    })
}
