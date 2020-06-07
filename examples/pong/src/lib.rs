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
}

impl Game {
    fn new() -> Self {
        let scene = Scene::new("#renderCanvas");
        let camera = Camera::new(&scene);
        let light_1 = HemisphericLight::new(&scene);
        let light_2 = PointLight::new(&scene);
        Game {
            scene,
            camera,
            shape: vec![],
            light_1,
            light_2,
        }
    }
}

#[no_mangle]
pub fn main() {
    babylon::js::log("Starting demo...");
    let mut game = GAME.lock().unwrap();
    for _ in 0..10 {
        let mut sphere = Sphere::new(&game.scene, babylon::js::random());
        sphere.set_position(
            babylon::js::random() - 0.5,
            babylon::js::random() - 0.5,
            babylon::js::random() - 0.5,
        );
        game.shape.push(sphere);
    }
}
