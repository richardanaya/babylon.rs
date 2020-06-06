pub use js_ffi::*;

pub struct Scene {
    scene_ref: JSValue
}

impl Scene {
    pub fn create_from_basic_engine(selector:&str) -> Scene {
        let func = register_function(r#"
            function(selector){
                var canvas = document.querySelector(selector);
                var engine = new BABYLON.Engine(canvas, true); 
                var createScene = function () {
                    var scene = new BABYLON.Scene(engine);
                    return scene;
                };
                var scene = createScene();
                engine.runRenderLoop(function () {
                        scene.render();
                });
                window.addEventListener("resize", function () {
                        engine.resize();
                });
                return scene;
            }
        "#);
        let scene_ref = func.invoke_1(selector);
        Scene {
            scene_ref
        }
    }
}
