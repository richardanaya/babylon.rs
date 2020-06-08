use crate::constants::*;
use crate::core::*;
use std::sync::Mutex;

lazy_static! {
    static ref GAME: Mutex<Option<Box<dyn BasicGame>>> = Mutex::new(None);
}

pub trait BasicGame: Send + Sync {
    fn get_scene(&self) -> &Scene;
    fn run(&mut self, _delta_time: f64) {}
    fn key_up(&mut self, _key_code: f64) {}
    fn key_down(&mut self, _key_code: f64) {}
}

pub fn run_basic_game<T>()
where
    T: 'static + BasicGame + Default + Send,
{
    let mut game = GAME.lock().unwrap();
    let t = T::default();
    let scene = t.get_scene();
    scene.add_before_render_observable(|| {
        let mut game = GAME.lock().unwrap();
        if let Some(g) = &mut *game {
            let scene = g.get_scene();
            let delta_time = scene.get_delta_time();
            g.run(delta_time / 1000.0);
        }
    });
    scene.add_keyboard_observable(|event_type, key_code| {
        let mut game = GAME.lock().unwrap();
        if let Some(g) = &mut *game {
            if event_type == KEYDOWN {
                g.key_down(key_code);
            }
            if event_type == KEYUP {
                g.key_up(key_code);
            }
        }
    });

    *game = Some(Box::new(t));
}
