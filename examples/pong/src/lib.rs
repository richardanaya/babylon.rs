use babylon::prelude::*;
#[macro_use]
extern crate lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref GAME: Mutex<Game> = Mutex::new(Game::new());
}

struct Game {
    scene: Scene,
    _camera: Camera,
    _light_1: HemisphericLight,
    _light_2: PointLight,
    ball: Sphere,
    paddle_1: Cube,
    paddle_2: Cube,
    paddle_dir: f64,
    ball_dir: Vector,
}

impl Game {
    fn new() -> Self {
        let mut scene = Scene::new("#renderCanvas");
        scene.set_clear_color(Color::new(0.0, 0.0, 0.0));
        let _camera = Camera::new(&scene);
        let _light_1 = HemisphericLight::new(&scene);
        let _light_2 = PointLight::new(&scene);
        let ball = Sphere::new(&scene, 0.05);
        let mut paddle_mat = StandardMaterial::new(&scene);
        paddle_mat.set_diffuse_color(Color::new(0.5, 0.5, 0.5));
        paddle_mat.set_emmisive_color(Color::new(0.5, 0.5, 0.5));
        let mut paddle_1 = Cube::new(&scene, 0.5, 0.05, 0.05);
        paddle_1.set_position(Vector::new(0.0, 0.5, 0.0));
        paddle_1.set_material(&paddle_mat);
        let mut paddle_2 = Cube::new(&scene, 0.5, 0.05, 0.05);
        paddle_2.set_position(Vector::new(0.0, -0.5, 0.0));
        paddle_2.set_material(&paddle_mat);
        Game {
            scene,
            _camera,
            _light_1,
            _light_2,
            ball,
            paddle_1,
            paddle_2,
            paddle_dir: 0.0,
            ball_dir: Vector::new(babylon::js::random() - 0.5, -1.0, 0.0),
        }
    }

    fn run(&mut self, delta_time: f64) {
        // get positions
        let p = self.paddle_1.get_position();
        let p2 = self.paddle_2.get_position();
        let bp = self.ball.get_position();

        // move ball
        let b_x = self.ball_dir.x * delta_time + bp.x;
        let b_y = self.ball_dir.y * delta_time + bp.y;
        if b_x > 0.5 || b_x < -0.5 {
            self.ball_dir.x = -self.ball_dir.x;
        }

        if b_y > 0.75 || b_y < -0.75 {
            self.ball.set_position(Vector::new(0.0, 0.0, 0.0));
            self.ball_dir.x = babylon::js::random() - 0.5;
            self.ball_dir.y = -self.ball_dir.y;
        } else if b_y > 0.45 || (b_y < -0.45 && b_x <= p2.x + 0.25 && b_x >= p2.x - 0.25) {
            self.ball_dir.y = -self.ball_dir.y;
            self.ball.set_position(Vector::new(b_x, b_y, 0.0));
        } else {
            self.ball.set_position(Vector::new(b_x, b_y, 0.0));
        }

        // move opponent paddle to match ball
        let x = b_x;
        let y = p.y;
        let z = p.z;
        self.paddle_1.set_position(Vector::new(x, y, z));

        // move paddle if it has velocity
        if self.paddle_dir != 0.0 {
            let x = p2.x;
            let y = p2.y;
            let z = p2.z;
            self.paddle_2
                .set_position(Vector::new(x + delta_time * self.paddle_dir, y, z));
        }
    }

    fn key_up(&mut self, _key_code: f64) {
        self.paddle_dir = 0.0;
    }

    fn key_down(&mut self, key_code: f64) {
        babylon::js::log(&format!("{}", key_code));
        if key_code == 37.0 {
            self.paddle_dir = 1.0;
        } else if key_code == 39.0 {
            self.paddle_dir = -1.0;
        }
    }
}

#[no_mangle]
pub fn main() {
    babylon::js::log("Starting demo...");
    let game = GAME.lock().unwrap();
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
