use crate::api::BabylonApi;
use crate::core::*;
use crate::math::*;
use js_ffi::*;

pub struct Sphere {
    position: Vector3<f32>,
    sphere_ref: JSObject,
}

impl Sphere {
    pub fn create_sphere(scene: &Scene, size: f32) -> Sphere {
        Sphere {
            position: Vector3::new(0.0, 0.0, 0.0),
            sphere_ref: BabylonApi::create_sphere(scene.get_js_ref(), size),
        }
    }

    pub fn set_position(&mut self, x: f32, y: f32, z: f32) {
        self.position.x = x;
        self.position.y = y;
        self.position.z = z;
        BabylonApi::set_position(&mut self.sphere_ref, x, y, z);
    }
}

impl Drop for Sphere {
    fn drop(&mut self) {
        BabylonApi::dispose_mesh(&mut self.sphere_ref);
        release_object(&self.sphere_ref)
    }
}
