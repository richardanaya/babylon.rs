use crate::api::BabylonApi;
use crate::core::*;
use crate::math::*;
use web::*;

pub trait Material {
    fn get_js_ref(&self) -> &ExternRef;
}

pub struct StandardMaterial {
    js_ref: ExternRef,
    diffuse_color: Color,
    specular_color: Color,
    emmisive_color: Color,
    ambient_color: Color,
    alpha: f64,
}

impl StandardMaterial {
    pub fn new(scene: &Scene) -> StandardMaterial {
        StandardMaterial {
            js_ref: BabylonApi::create_standard_material(scene.get_js_ref()),
            diffuse_color: Color::new(0.0, 0.0, 0.0),
            specular_color: Color::new(0.0, 0.0, 0.0),
            emmisive_color: Color::new(0.0, 0.0, 0.0),
            ambient_color: Color::new(0.0, 0.0, 0.0),
            alpha: 1.0,
        }
    }

    pub fn set_diffuse_color(&mut self, c: Color) {
        self.diffuse_color = c;
        BabylonApi::set_diffuse_color(self.get_js_ref(), c.x, c.y, c.z);
    }

    pub fn set_emmisive_color(&mut self, c: Color) {
        self.emmisive_color = c;
        BabylonApi::set_emmisive_color(self.get_js_ref(), c.x, c.y, c.z);
    }

    pub fn set_specular_color(&mut self, c: Color) {
        self.specular_color = c;
        BabylonApi::set_specular_color(self.get_js_ref(), c.x, c.y, c.z);
    }

    pub fn set_ambient_color(&mut self, c: Color) {
        self.ambient_color = c;
        BabylonApi::set_ambient_color(self.get_js_ref(), c.x, c.y, c.z);
    }

    pub fn set_alpha(&mut self, a: f64) {
        self.alpha = a;
        BabylonApi::set_alpha(self.get_js_ref(), a);
    }
}

impl Material for StandardMaterial {
    fn get_js_ref(&self) -> &ExternRef {
        return &self.js_ref;
    }
}
