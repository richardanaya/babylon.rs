use babylon::prelude::*;

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

impl Default for Game {
    fn default() -> Self {
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
}

impl BasicGame for Game {
    fn get_scene(&self) -> &Scene {
        &self.scene
    }

    fn run(&mut self, delta_time: f64) {
        // get positions
        let p2 = self.paddle_2.get_position();
        let bp = self.ball.get_position();

        // move ball
        let mut b_x = self.ball_dir.x * delta_time + bp.x;
        let mut b_y = self.ball_dir.y * delta_time + bp.y;
        if b_x > 0.5 || b_x < -0.5 {
            self.ball_dir.x = -self.ball_dir.x;
        }
        if b_y > 0.75 || b_y < -0.75 {
            // Reset ball if outside play area
            b_x = 0.0;
            b_y = 0.0;
            self.ball_dir.x = babylon::js::random() - 0.5;
            self.ball_dir.y = -self.ball_dir.y;
        } else if b_y > 0.45 || (b_y < -0.45 && b_y > -0.55 && b_x <= p2.x + 0.25 && b_x >= p2.x - 0.25) {
            // Hit paddle (top paddle is assumed to always hit)
            self.ball_dir.y = -self.ball_dir.y;
            b_y = if b_y > 0.0 { 0.44 } else { -0.44 };
        }
        
        self.ball.set_position(Vector::new(b_x, b_y, 0.0));
        
        // move opponent paddle to match ball
        self.paddle_1.set_position_x(b_x);

        // move paddle if it has velocity
        let p2_x = p2.x + delta_time * self.paddle_dir;
        self.paddle_2.set_position_x(p2_x);
    }

    fn key_up(&mut self, _key_code: f64) {
        self.paddle_dir = 0.0;
    }

    fn key_down(&mut self, key_code: f64) {
        match key_code {
            37.0 => self.paddle_dir = 1.0,
            39.0 => self.paddle_dir = -1.0,
            _ => {}
        }
    }
}

#[no_mangle]
pub fn main() {
    run_basic_game::<Game>();
}
