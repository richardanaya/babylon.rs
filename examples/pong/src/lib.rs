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
    camera: Camera,
    light_1: HemisphericLight,
    light_2: PointLight,
    ball: Sphere,
}

impl Game {
    fn new() -> Self {
        let scene = Scene::new("#renderCanvas");
        let camera = Camera::new(&scene);
        let light_1 = HemisphericLight::new(&scene);
        let light_2 = PointLight::new(&scene);
        let ball = Sphere::new(&scene, 0.1);
        Game {
            scene,
            camera,
            shape: vec![],
            light_1,
            light_2,
            ball,
        }
    }

    fn run(&mut self, delta_time: f64) {
        let p = self.ball.get_position();
        self.ball
            .set_position(p + Vector::new(0.0, -1.0 * delta_time, 0.0))
    }

    fn key_down(&mut self, key_code: f64) {}
}

#[no_mangle]
pub fn main() {
    babylon::js::log("Starting demo...");
    let mut game = GAME.lock().unwrap();
    game.scene.add_before_render_observable(|| {
        let mut game = GAME.lock().unwrap();
        let delta_time = game.scene.get_delta_time();
        game.run(delta_time / 1000.0);
    });
    game.scene.add_keyboard_observable(|event_type, key_code| {
        if event_type == KEYUP {
            let mut game = GAME.lock().unwrap();
            game.key_down(key_code);
        }
    })
}
