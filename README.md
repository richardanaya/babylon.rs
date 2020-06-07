# Babylon.rs

<a href="https://docs.rs/babylon"><img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square" alt="docs.rs docs" /></a>

WebAssembly wrapper for [Babylon.js](https://www.babylonjs.com/) in Rust.

This project is pre-alpha and the api is in active exploration. Current priorities:

* get a basic GLTF up
* get a camera
* get some sort of interaction

This project uses [`js_ffi`](https://github.com/richardanaya/js_ffi) for binding and [`globals`](https://github.com/richardanaya/globals) for global static singletons.

# HelloWorld

<p align="center">
  <img src="https://richardanaya.github.io/babylon.rs/images/demo_0.png">
</p>

```rust
use babylon::*;

struct Game {
    scene: Scene,
    shape: Option<Sphere>,
}

impl Default for Game {
    fn default() -> Self {
        Game {
            scene: Scene::create_from_basic_engine("#renderCanvas"),
            shape: None,
        }
    }
}

#[no_mangle]
pub fn main() {
    let mut game = globals::<Game>::get();
    game.shape = Some(Sphere::create_sphere(&self.scene, 1.0));
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
