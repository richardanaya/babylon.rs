#![no_std]

use crate::api::BabylonApi;

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
            scene_ref: BabylonApi::create_basic_scene(selector),
        }
    }
}

impl Drop for Scene {
    fn drop(&mut self){
        release_object(&self.scene_ref)
    }
}


impl Sphere {
    pub fn create_sphere(scene: &Scene, size: f32) -> Sphere {
        Sphere {
            sphere_ref: BabylonApi::create_sphere(&scene.scene_ref, size),
        }
    }
}

impl Drop for Sphere {
    fn drop(&mut self){
        BabylonApi::dispose_mesh(&mut self.sphere_ref);
        release_object(&self.sphere_ref)
    }
}
