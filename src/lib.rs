#![no_std]

use crate::api::BabylonApi;

pub use js_ffi::*;

mod api;

struct Vector {
    x: f32,
    y: f32,
    z: f32,
}

pub struct Scene {
    scene_ref: JSObject,
}

pub struct Sphere {
    position: Vector,
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
    fn drop(&mut self) {
        release_object(&self.scene_ref)
    }
}

impl Sphere {
    pub fn create_sphere(scene: &Scene, size: f32) -> Sphere {
        Sphere {
            position: Vector {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            sphere_ref: BabylonApi::create_sphere(&scene.scene_ref, size),
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

pub mod js;
