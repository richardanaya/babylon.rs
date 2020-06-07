#[macro_use]
extern crate lazy_static;
use babylon::*;

lazy_static! {
    static ref SCENE: std::sync::Mutex<Scene> = {
        let scene = Scene::create_from_basic_engine("#renderCanvas");
        std::sync::Mutex::new(scene)
    };
}

#[no_mangle]
pub fn main() {
    let mut scene = SCENE.lock().unwrap();
}
