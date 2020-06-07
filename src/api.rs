use js_ffi::*;

pub struct BabylonApi {
    fn_log: JSInvoker,
    fn_error: JSInvoker,
    fn_debug: JSInvoker,
    fn_random: JSInvoker,
    fn_create_basic_scene: JSInvoker,
    fn_create_sphere: JSInvoker,
    fn_dispose_mesh: JSInvoker,
    fn_set_position: JSInvoker,
}

impl Default for BabylonApi {
    fn default() -> Self {
        BabylonApi {
            fn_create_basic_scene: register_function(
                r#"
                function(selector){
                    var canvas = document.querySelector(selector);
                    var engine = new BABYLON.Engine(canvas, true); 
                    var createScene = function () {
                        var scene = new BABYLON.Scene(engine);
    
                        // Add a camera to the scene and attach it to the canvas
                        var camera = new BABYLON.ArcRotateCamera(
                            "Camera",
                            Math.PI / 2,
                            Math.PI / 2,
                            2,
                            BABYLON.Vector3.Zero(),
                            scene
                        );
                        camera.attachControl(canvas, true);
    
                        // Add lights to the scene
                        var light1 = new BABYLON.HemisphericLight(
                            "light1",
                            new BABYLON.Vector3(1, 1, 0),
                            scene
                        );
                        var light2 = new BABYLON.PointLight(
                            "light2",
                            new BABYLON.Vector3(0, 1, -1),
                            scene
                        );
    
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
            "#,
            ),
            fn_create_sphere: register_function(
                r#"
                function(scene,size){
                    return BABYLON.MeshBuilder.CreateSphere(
                        "sphere",
                        { diameter: size },
                        scene);
                }
            "#,
            ),
            fn_dispose_mesh: register_function(
                r#"
                function(mesh){
                    mesh.dispose()
                }
            "#,
            ),
            fn_log: register_function(
                r#"
                function(msg){
                    console.log(msg)
                }
            "#,
            ),
            fn_error: register_function(
                r#"
                function(msg){
                    console.error(msg)
                }
            "#,
            ),
            fn_debug: register_function(
                r#"
                function(){
                    debugger;
                }
            "#,
            ),
            fn_random: register_function(
                r#"
                function(){
                    return Math.random()
            "#,
            ),
            fn_set_position: register_function(
                r#"
                function(mesh,x,y,z){
                    mesh.position = new BABYLON.Vector3(x,y,z)
                }
            "#,
            ),
        }
    }
}

impl BabylonApi {
    pub fn create_basic_scene(selector: &str) -> JSObject {
        let api = globals::get::<BabylonApi>();
        api.fn_create_basic_scene.invoke_1(selector).to_js_object()
    }

    pub fn create_sphere(scene_ref: &JSObject, size: f32) -> JSObject {
        let api = globals::get::<BabylonApi>();
        api.fn_create_sphere
            .invoke_2(scene_ref, size)
            .to_js_object()
    }

    pub fn dispose_mesh(mesh: &JSObject) {
        let api = globals::get::<BabylonApi>();
        api.fn_dispose_mesh.invoke_1(mesh);
    }

    pub fn log(msg: &str) {
        let api = globals::get::<BabylonApi>();
        api.fn_log.invoke_1(msg);
    }

    pub fn error(msg: &str) {
        let api = globals::get::<BabylonApi>();
        api.fn_error.invoke_1(msg);
    }

    pub fn debugger() {
        let api = globals::get::<BabylonApi>();
        api.fn_debug.invoke_0();
    }

    pub fn random() -> f32 {
        let api = globals::get::<BabylonApi>();
        api.fn_random.invoke_0() as f32
    }

    pub fn set_position(mesh: &JSObject, x: f32, y: f32, z: f32) -> JSObject {
        let api = globals::get::<BabylonApi>();
        api.fn_set_position.invoke_4(mesh, x, y, z).to_js_object()
    }
}
