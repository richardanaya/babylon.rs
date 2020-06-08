use crate::api::BabylonApi;
use crate::core::Scene;
use js_ffi::*;

pub struct HemisphericLight {
    _js_ref: JSObject,
}

impl HemisphericLight {
    pub fn new(scene: &Scene) -> HemisphericLight {
        HemisphericLight {
            _js_ref: BabylonApi::create_hemispheric_light(&scene.get_js_ref()),
        }
    }
}

pub struct PointLight {
    _js_ref: JSObject,
}

impl PointLight {
    pub fn new(scene: &Scene) -> PointLight {
        PointLight {
            _js_ref: BabylonApi::create_point_light(&scene.get_js_ref()),
        }
    }
}
