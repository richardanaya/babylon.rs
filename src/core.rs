use crate::api::BabylonApi;
use js_ffi::*;

pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub struct Scene {
    scene_ref: JSObject,
}

impl Scene {
    pub fn create_from_basic_engine(selector: &str) -> Scene {
        Scene {
            scene_ref: BabylonApi::create_basic_scene(selector),
        }
    }

    pub fn get_js_ref(&self) -> &JSObject {
        &self.scene_ref
    }
}

impl Drop for Scene {
    fn drop(&mut self) {
        release_object(&self.scene_ref)
    }
}
