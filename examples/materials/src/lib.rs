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
        let mut cube = Cube::new(
            &game.scene,
            babylon::js::random(),
            babylon::js::random(),
            babylon::js::random(),
        );
        let mut mat = StandardMaterial::new(&game.scene);
        mat.set_diffuse_color(Color::new(babylon::js::random(),babylon::js::random(),babylon::js::random()));
        mat.set_alpha(babylon::js::random());
        cube.set_material(&mat);
        cube.set_position(
            babylon::js::random() - 0.5,
            babylon::js::random() - 0.5,
            babylon::js::random() - 0.5,
        );
        game.shape.push(cube);
    }
}
