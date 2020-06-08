# babylon.rs

<a href="https://docs.rs/babylon"><img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square" alt="docs.rs docs" /></a>

A WebAssembly wrapper for [babylon.js](https://www.babylonjs.com/) in Rust.

This project is pre-alpha and the api is in active exploration. Current priorities:

* get a basic GLTF up
* get a camera
* get some sort of interaction

This project uses [`js_ffi`](https://github.com/richardanaya/js_ffi) for javascript binding and [`lazy_static`](https://github.com/rust-lang-nursery/lazy-static.rs) for global static singletons fairly extensively.

# Idioms
* Scenes hold 3D objects
* Materials determine how a 3D object looks
* When an 3D object drops it's removed from the scene

# HelloWorld

<p align="center">
  <img src="https://richardanaya.github.io/babylon.rs/images/demo_0.png">
</p>

```rust
use babylon::prelude::*;
#[macro_use]
extern crate lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref GAME: Mutex<Game> = Mutex::new(Game::new());
}

struct Game {
    scene: Scene,
    shape: Vec<Sphere>,
}

impl Game {
    fn new() -> Self {
        Game {
            scene: Scene::create_from_basic_engine("#renderCanvas"),
            shape: vec![],
        }
    }
}

#[no_mangle]
pub fn main() {
    babylon::js::log("Starting demo...");
    let mut game = GAME.lock().unwrap();
    for _ in 0..10 {
        let mut sphere = Sphere::new(&game.scene, babylon::js::random());
        sphere.set_position(Vector::new(
            babylon::js::random() - 0.5,
            babylon::js::random() - 0.5,
            babylon::js::random() - 0.5,
        ));
        game.shape.push(sphere);
    }
}
```

See this demo [here](https://richardanaya.github.io/babylon.rs/examples/helloworld/index.html) ( be sure to play with mouse and arrow keys :arrow_left: :arrow_up: :arrow_down: :arrow_right:!)

# Materials

<p align="center">
  <img src="https://richardanaya.github.io/babylon.rs/images/demo_1.png">
</p>

```rust
let mut game = GAME.lock().unwrap();
for _ in 0..10 {
    let mut cube = Cube::new(
        &game.scene,
        babylon::js::random(),
        babylon::js::random(),
        babylon::js::random(),
    );
    let mut mat = StandardMaterial::new(&game.scene);
    mat.set_diffuse_color(Color::new(babylon::js::random(),babylon::js::random(),babylon::js::random()));
    mat.set_alpha(babylon::js::random());
    cube.set_material(&mat);
    cube.set_position(Vector::new(
        babylon::js::random() - 0.5,
        babylon::js::random() - 0.5,
        babylon::js::random() - 0.5,
    ));
    game.shape.push(cube);
}
```

See this demo [here](https://richardanaya.github.io/babylon.rs/examples/materials/index.html) ( be sure to play with mouse and arrow keys :arrow_left: :arrow_up: :arrow_down: :arrow_right:!)

# Pong in 100 Lines

```rust
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
        self.paddle_1.set_position_x(b_x);

        // move paddle if it has velocity
        if self.paddle_dir != 0.0 {
            let p2_x = p2.x + delta_time * self.paddle_dir;
            self.paddle_2.set_position_x(p2_x)
        }
    }

    fn key_up(&mut self, _key_code: f64) {
        self.paddle_dir = 0.0;
    }

    fn key_down(&mut self, key_code: f64) {
        if key_code == 37.0 {
            self.paddle_dir = 1.0;
        } else if key_code == 39.0 {
            self.paddle_dir = -1.0;
        }
    }
}

#[no_mangle]
pub fn main() {
    run_basic_game::<Game>();
}
```

# Other Demos

* [Timer](https://richardanaya.github.io/babylon.rs/examples/timer/index.html)
* [Keyboard](https://richardanaya.github.io/babylon.rs/examples/keyboard/index.html)

# License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in `babylon.rs` by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
