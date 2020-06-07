pub use js_ffi::*;

mod api;

pub struct Scene {
    scene_ref: JSObject,
}

pub struct Sphere {
    sphere_ref: JSObject,
}

impl Scene {
    pub fn create_from_basic_engine(selector: &str) -> Scene {
        Scene {
            scene_ref: crate::api::BabylonApi::create_basic_scene(selector),
        }
    }
}

impl Sphere {
    pub fn create_sphere(scene: &Scene, size: f32) -> Sphere {
        Sphere {
            sphere_ref: crate::api::BabylonApi::create_sphere(&scene.scene_ref, size),
        }
    }
}
