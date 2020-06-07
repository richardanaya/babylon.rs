use crate::api::BabylonApi;
use crate::core::*;
use crate::math::*;
use js_ffi::*;

pub struct Sphere {
    position: Vector3<f32>,
    js_ref: JSObject,
}

impl Sphere {
    pub fn new(scene: &Scene, size: f32) -> Sphere {
        Sphere {
            position: Vector3::new(0.0, 0.0, 0.0),
            js_ref: BabylonApi::create_sphere(scene.get_js_ref(), size),
        }
    }

    pub fn set_position(&mut self, x: f32, y: f32, z: f32) {
        self.position.x = x;
        self.position.y = y;
        self.position.z = z;
        BabylonApi::set_position(&mut self.js_ref, x, y, z);
    }
}

impl Drop for Sphere {
    fn drop(&mut self) {
        BabylonApi::dispose_mesh(&mut self.js_ref);
        release_object(&self.js_ref)
    }
}

pub struct Cube {
    position: Vector3<f32>,
    js_ref: JSObject,
}

impl Cube {
    pub fn new(scene: &Scene, width: f32, height: f32, depth: f32) -> Sphere {
        Sphere {
            position: Vector3::new(0.0, 0.0, 0.0),
            js_ref: BabylonApi::create_cube(scene.get_js_ref(), width, height, depth),
        }
    }

    pub fn set_position(&mut self, x: f32, y: f32, z: f32) {
        self.position.x = x;
        self.position.y = y;
        self.position.z = z;
        BabylonApi::set_position(&mut self.js_ref, x, y, z);
    }
}

impl Drop for Cube {
    fn drop(&mut self) {
        BabylonApi::dispose_mesh(&mut self.js_ref);
        release_object(&self.js_ref)
    }
}
