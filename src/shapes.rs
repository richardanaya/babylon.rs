use crate::api::BabylonApi;
use crate::core::*;
use crate::materials::*;
use crate::math::*;
use web::*;

pub struct Sphere {
    position: Vector3<f64>,
    js_ref: ExternRef,
}

impl Sphere {
    pub fn new(scene: &Scene, size: f64) -> Sphere {
        Sphere {
            position: Vector3::new(0.0, 0.0, 0.0),
            js_ref: BabylonApi::create_sphere(scene.get_js_ref(), size),
        }
    }

    pub fn get_position(&self) -> &Vector {
        &self.position
    }

    pub fn set_position(&mut self, p: Vector) {
        self.position = p;
        BabylonApi::set_position(
            &mut self.js_ref,
            self.position.x,
            self.position.y,
            self.position.z,
        );
    }

    pub fn set_position_x(&mut self, v: f64) {
        self.position.x = v;
        BabylonApi::set_position(
            &mut self.js_ref,
            self.position.x,
            self.position.y,
            self.position.z,
        );
    }

    pub fn set_position_y(&mut self, v: f64) {
        self.position.y = v;
        BabylonApi::set_position(
            &mut self.js_ref,
            self.position.x,
            self.position.y,
            self.position.z,
        );
    }

    pub fn set_position_z(&mut self, v: f64) {
        self.position.z = v;
        BabylonApi::set_position(
            &mut self.js_ref,
            self.position.x,
            self.position.y,
            self.position.z,
        );
    }

    pub fn set_material<T>(&mut self, mat: &T)
    where
        T: Material,
    {
        BabylonApi::set_material(&mut self.js_ref, mat.get_js_ref());
    }
}

impl Drop for Sphere {
    fn drop(&mut self) {
        BabylonApi::dispose_mesh(&mut self.js_ref);
    }
}

pub struct Cube {
    position: Vector3<f64>,
    js_ref: ExternRef,
}

impl Cube {
    pub fn new(scene: &Scene, width: f64, height: f64, depth: f64) -> Cube {
        Cube {
            position: Vector3::new(0.0, 0.0, 0.0),
            js_ref: BabylonApi::create_cube(scene.get_js_ref(), width, height, depth),
        }
    }

    pub fn get_position(&self) -> &Vector {
        &self.position
    }

    pub fn set_position(&mut self, p: Vector) {
        self.position = p;
        BabylonApi::set_position(&mut self.js_ref, p.x, p.y, p.z);
    }

    pub fn set_position_x(&mut self, v: f64) {
        self.position.x = v;
        BabylonApi::set_position(
            &mut self.js_ref,
            self.position.x,
            self.position.y,
            self.position.z,
        );
    }

    pub fn set_position_y(&mut self, v: f64) {
        self.position.y = v;
        BabylonApi::set_position(
            &mut self.js_ref,
            self.position.x,
            self.position.y,
            self.position.z,
        );
    }

    pub fn set_position_z(&mut self, v: f64) {
        self.position.z = v;
        BabylonApi::set_position(
            &mut self.js_ref,
            self.position.x,
            self.position.y,
            self.position.z,
        );
    }

    pub fn set_material<T>(&mut self, mat: &T)
    where
        T: Material,
    {
        BabylonApi::set_material(&mut self.js_ref, mat.get_js_ref());
    }
}

impl Drop for Cube {
    fn drop(&mut self) {
        BabylonApi::dispose_mesh(&mut self.js_ref);
    }
}

pub struct GLTF {
    position: Vector3<f64>,
    js_ref: ExternRef,
}

impl GLTF {
    pub fn new(scene: &Scene, file: &str) -> GLTF {
        GLTF {
            position: Vector3::new(0.0, 0.0, 0.0),
            js_ref: BabylonApi::create_gltf(scene.get_js_ref(), file),
        }
    }

    pub fn get_position(&self) -> &Vector {
        &self.position
    }

    pub fn set_position(&mut self, p: Vector) {
        self.position = p;
        BabylonApi::set_position(&mut self.js_ref, p.x, p.y, p.z);
    }

    pub fn set_position_x(&mut self, v: f64) {
        self.position.x = v;
        BabylonApi::set_position(
            &mut self.js_ref,
            self.position.x,
            self.position.y,
            self.position.z,
        );
    }

    pub fn set_position_y(&mut self, v: f64) {
        self.position.y = v;
        BabylonApi::set_position(
            &mut self.js_ref,
            self.position.x,
            self.position.y,
            self.position.z,
        );
    }

    pub fn set_position_z(&mut self, v: f64) {
        self.position.z = v;
        BabylonApi::set_position(
            &mut self.js_ref,
            self.position.x,
            self.position.y,
            self.position.z,
        );
    }

    pub fn set_scaling(&mut self, p: Vector) {
        self.position = p;
        BabylonApi::set_scaling(&mut self.js_ref, p.x, p.y, p.z);
    }
}

impl Drop for GLTF {
    fn drop(&mut self) {
        BabylonApi::dispose_mesh(&mut self.js_ref);
    }
}
