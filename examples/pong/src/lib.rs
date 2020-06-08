use babylon::prelude::*;
#[macro_use]
extern crate lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref GAME: Mutex<Game> = Mutex::new(Game::new());
}

struct Game {
    scene: Scene,
    camera: Camera,
    light_1: HemisphericLight,
    light_2: PointLight,
    ball: Sphere,
    paddle_1: Cube,
    paddle_2: Cube,
    dir: f64,
}

impl Game {
    fn new() -> Self {
        let mut scene = Scene::new("#renderCanvas");
        scene.set_clear_color(Color::new(0.0, 0.0, 0.0));
        let camera = Camera::new(&scene);
        let light_1 = HemisphericLight::new(&scene);
        let light_2 = PointLight::new(&scene);
        let ball = Sphere::new(&scene, 0.05);
        let mut paddle_1 = Cube::new(&scene, 0.5, 0.05, 0.05);
        paddle_1.set_position(Vector::new(0.0, 0.5, 0.0));
        let mut paddle_2 = Cube::new(&scene, 0.5, 0.05, 0.05);
        paddle_2.set_position(Vector::new(0.0, -0.5, 0.0));
        Game {
            scene,
            camera,
            light_1,
            light_2,
            ball,
            paddle_1,
            paddle_2,
            dir: 0.0,
        }
    }

    fn run(&mut self, delta_time: f64) {
        let p = self.ball.get_position();
        self.ball
            .set_position(p + Vector::new(0.0, -1.0 * delta_time, 0.0));
        if self.dir != 0.0 {
            let p = self.paddle_2.get_position();
            self.paddle_2
                .set_position(p + Vector::new(delta_time * self.dir, 0.0, 0.0))
        }
    }

    fn key_up(&mut self, key_code: f64) {
        self.dir = 0.0;
    }

    fn key_down(&mut self, key_code: f64) {
        babylon::js::log(&format!("{}", key_code));
        if key_code == 37.0 {
            self.dir = 1.0;
        } else if key_code == 39.0 {
            self.dir = -1.0;
        }
    }
}

#[no_mangle]
pub fn main() {
    babylon::js::log("Starting demo...");
    let mut game = GAME.lock().unwrap();
    game.scene.add_before_render_observable(|| {
        let mut game = GAME.lock().unwrap();
        let delta_time = game.scene.get_delta_time();
        game.run(delta_time / 1000.0);
    });
    game.scene.add_keyboard_observable(|event_type, key_code| {
        let mut game = GAME.lock().unwrap();
        if event_type == KEYDOWN {
            game.key_down(key_code);
        }
        if event_type == KEYUP {
            game.key_up(key_code);
        }
    })
}
