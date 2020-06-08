use crate::api::BabylonApi;
use crate::math::*;
use alloc::boxed::Box;
use js_ffi::*;

pub struct Scene {
    ambient_color: Color,
    clear_color: Color,
    scene_ref: JSObject,
}

impl Scene {
    pub fn new(selector: &str) -> Scene {
        let scene_ref = BabylonApi::create_scene(selector);
        Scene {
            clear_color: Color::new(0.0, 0.0, 0.0),
            ambient_color: Color::new(0.0, 0.0, 0.0),
            scene_ref,
        }
    }

    pub fn create_from_basic_engine(selector: &str) -> Scene {
        let scene_ref = BabylonApi::create_basic_scene(selector);
        Scene {
            clear_color: Color::new(0.0, 0.0, 0.0),
            ambient_color: Color::new(0.0, 0.0, 0.0),
            scene_ref,
        }
    }

    pub fn get_js_ref(&self) -> &JSObject {
        &self.scene_ref
    }

    pub fn add_keyboard_observable<T>(&self, callback: T)
    where
        T: 'static + FnMut(JSValue, JSValue) -> () + Send,
    {
        BabylonApi::add_keyboard_observable(&self.scene_ref, Box::new(callback));
    }

    pub fn add_before_render_observable<T>(&self, callback: T)
    where
        T: 'static + FnMut() -> () + Send,
    {
        BabylonApi::add_observable(
            &self.scene_ref,
            "onBeforeRenderObservable",
            Box::new(callback),
        );
    }

    pub fn get_delta_time(&self) -> f64 {
        BabylonApi::get_delta_time(&self.scene_ref)
    }

    pub fn set_ambient_color(&mut self, c: Color) {
        self.ambient_color = c;
        BabylonApi::set_ambient_color(self.get_js_ref(), c.x, c.y, c.z);
    }

    pub fn set_clear_color(&mut self, c: Color) {
        self.clear_color = c;
        BabylonApi::set_clear_color(self.get_js_ref(), c.x, c.y, c.z);
    }
}

impl Drop for Scene {
    fn drop(&mut self) {
        release_object(&self.scene_ref)
    }
}
