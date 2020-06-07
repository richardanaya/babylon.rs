use crate::api::BabylonApi;
use js_ffi::*;
use crate::core::Scene;

pub struct Camera {
    js_ref: JSObject,
}

impl Camera {
    pub fn new(scene: &Scene) -> Camera {
        Camera {
            js_ref: BabylonApi::create_arc_rotate_camera(&scene.get_js_ref()),
        }
    }
}
