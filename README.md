# Babylon.rs

<a href="https://docs.rs/babylon"><img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square" alt="docs.rs docs" /></a>

WebAssembly wrapper for [Babylon.js](https://www.babylonjs.com/)

This project uses [`js_ffi`](https://github.com/richardanaya/js_ffi) for binding.

# HelloWorld

```rust
use babylon::*;

#[macro_use]
extern crate lazy_static;

use std::sync::Mutex;

lazy_static! {
    static ref GAME: Mutex<Game> = Mutex::new(Game::new());
}

struct Game {
    scene: Scene,
    shape: Option<Sphere>,
}

impl Game {
    fn new() -> Game {
        Game {
            scene: Scene::create_from_basic_engine("#renderCanvas"),
            shape: None,
        }
    }

    fn init(&mut self) {
        self.shape = Some(Sphere::create_sphere(&self.scene, 1.0));
    }
}

#[no_mangle]
pub fn main() {
    let mut game = GAME.lock().unwrap();
    game.init();
}
```

See this demo [here](https://richardanaya.github.io/babylon.rs/examples/helloworld/index.html)


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
