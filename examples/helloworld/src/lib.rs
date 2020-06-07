use babylon::*;

struct Game {
    scene: Scene,
    shape: Option<Sphere>,
}

impl Default for Game {
    fn default() -> Self {
        Game {
            scene: Scene::create_from_basic_engine("#renderCanvas"),
            shape: None,
        }
    }
}

#[no_mangle]
pub fn main() {
    let mut game = globals::get::<Game>();
    game.shape = Some(Sphere::create_sphere(&game.scene, 1.0));
}