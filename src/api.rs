use alloc::boxed::Box;
use js_ffi::*;

pub struct BabylonApi {
    fn_log: JSInvoker,
    fn_error: JSInvoker,
    fn_debug: JSInvoker,
    fn_random: JSInvoker,
    fn_create_basic_scene: JSInvoker,
    fn_create_scene: JSInvoker,
    fn_create_sphere: JSInvoker,
    fn_create_cube: JSInvoker,
    fn_create_standard_material: JSInvoker,
    fn_dispose_mesh: JSInvoker,
    fn_set_position: JSInvoker,
    fn_set_material: JSInvoker,
    fn_set_emmisive_color: JSInvoker,
    fn_set_diffuse_color: JSInvoker,
    fn_set_specular_color: JSInvoker,
    fn_set_ambient_color: JSInvoker,
    fn_set_clear_color: JSInvoker,
    fn_set_alpha: JSInvoker,
    fn_add_keyboard_observable: JSInvoker,
    fn_add_observable: JSInvoker,
    fn_get_delta_time: JSInvoker,
    fn_create_arc_rotate_camera: JSInvoker,
    fn_create_hemispheric_light: JSInvoker,
    fn_create_point_light: JSInvoker,
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
            fn_create_scene: register_function(
                r#"
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
            "#,
            ),
            fn_create_sphere: register_function(
                r#"
                function(scene,size){
                    return BABYLON.MeshBuilder.CreateSphere(
                        null,
                        { diameter: size },
                        scene);
                }
            "#,
            ),
            fn_create_cube: register_function(
                r#"
                function(scene,w,h,d){
                    return BABYLON.MeshBuilder.CreateBox(
                        null,
                        { height: h, width: w, depth: d },
                        scene);
                }
            "#,
            ),
            fn_create_standard_material: register_function(
                r#"
                function(scene){
                    return new BABYLON.StandardMaterial(null, scene);
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
                    console.log(msg);
                }
            "#,
            ),
            fn_error: register_function(
                r#"
                function(msg){
                    console.error(msg);
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
                    return Math.random();
                }
            "#,
            ),
            fn_set_position: register_function(
                r#"
                function(mesh,x,y,z){
                    mesh.position = new BABYLON.Vector3(x,y,z);
                }
            "#,
            ),
            fn_set_material: register_function(
                r#"
                function(mesh,mat){
                    mesh.material = mat;
                }
            "#,
            ),
            fn_set_emmisive_color: register_function(
                r#"
                function(mat,r,g,b){
                    mat.emissiveColor = new BABYLON.Color3(r, g, b);
                }
            "#,
            ),
            fn_set_diffuse_color: register_function(
                r#"
                function(mat,r,g,b){
                    mat.diffuseColor = new BABYLON.Color3(r, g, b);
                }
            "#,
            ),
            fn_set_specular_color: register_function(
                r#"
                function(mat,r,g,b){
                    mat.specularColor = new BABYLON.Color3(r, g, b);
                }
            "#,
            ),
            fn_set_ambient_color: register_function(
                r#"
                function(mat,r,g,b){
                    mat.ambientColor = new BABYLON.Color3(r, g, b);
                }
            "#,
            ),
            fn_set_clear_color: register_function(
                r#"
                function(scene,r,g,b){
                    scene.clearColor = new BABYLON.Color3(r, g, b);
                }
            "#,
            ),
            fn_set_alpha: register_function(
                r#"
                function(mat,a){
                    mat.alpha = a;
                }
            "#,
            ),
            fn_add_keyboard_observable: register_function(
                r#"
                function(scene,cb){
                    scene.onKeyboardObservable.add((kbInfo) => {
                        cb(kbInfo.type,kbInfo.event.keyCode)
                    });
                }
            "#,
            ),
            fn_add_observable: register_function(
                r#"
                function(scene,name,cb){
                    scene[name].add(() => {
                        cb()
                    });
                }
            "#,
            ),
            fn_get_delta_time: register_function(
                r#"
                function(scene){
                    return scene.getEngine().getDeltaTime();
                }
            "#,
            ),
            fn_create_arc_rotate_camera: register_function(
                r#"
                function(scene){
                    var camera = new BABYLON.ArcRotateCamera(
                        "Camera",
                        Math.PI / 2,
                        Math.PI / 2,
                        2,
                        BABYLON.Vector3.Zero(),
                        scene
                    );
                    return camera;
                }
            "#,
            ),
            fn_create_hemispheric_light: register_function(
                r#"
                function(scene){
                    var light = new BABYLON.HemisphericLight(
                        null,
                        new BABYLON.Vector3(1, 1, 0),
                        scene
                    );
                    return light;
                }
            "#,
            ),
            fn_create_point_light: register_function(
                r#"
                function(scene){
                    var light = new BABYLON.PointLight(
                        null,
                        new BABYLON.Vector3(0, 1, -1),
                        scene
                    );
                    return light;
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
    pub fn create_scene(selector: &str) -> JSObject {
        let api = globals::get::<BabylonApi>();
        api.fn_create_scene.invoke_1(selector).to_js_object()
    }
    pub fn create_sphere(scene_ref: &JSObject, size: f64) -> JSObject {
        let api = globals::get::<BabylonApi>();
        api.fn_create_sphere
            .invoke_2(scene_ref, size)
            .to_js_object()
    }

    pub fn create_cube(scene_ref: &JSObject, width: f64, height: f64, depth: f64) -> JSObject {
        let api = globals::get::<BabylonApi>();
        api.fn_create_cube
            .invoke_4(scene_ref, width, height, depth)
            .to_js_object()
    }

    pub fn create_standard_material(scene_ref: &JSObject) -> JSObject {
        let api = globals::get::<BabylonApi>();
        api.fn_create_standard_material
            .invoke_1(scene_ref)
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

    pub fn random() -> f64 {
        let api = globals::get::<BabylonApi>();
        api.fn_random.invoke_0()
    }

    pub fn set_position(mesh: &JSObject, x: f64, y: f64, z: f64) {
        let api = globals::get::<BabylonApi>();
        api.fn_set_position.invoke_4(mesh, x, y, z);
    }

    pub fn set_material(mesh: &JSObject, mat: &JSObject) {
        let api = globals::get::<BabylonApi>();
        api.fn_set_material.invoke_2(mesh, mat);
    }

    pub fn set_emmisive_color(mat: &JSObject, r: f64, g: f64, b: f64) {
        let api = globals::get::<BabylonApi>();
        api.fn_set_emmisive_color.invoke_4(mat, r, g, b);
    }

    pub fn set_diffuse_color(mat: &JSObject, r: f64, g: f64, b: f64) {
        let api = globals::get::<BabylonApi>();
        api.fn_set_diffuse_color.invoke_4(mat, r, g, b);
    }

    pub fn set_specular_color(mat: &JSObject, r: f64, g: f64, b: f64) {
        let api = globals::get::<BabylonApi>();
        api.fn_set_specular_color.invoke_4(mat, r, g, b);
    }

    pub fn set_ambient_color(mat: &JSObject, r: f64, g: f64, b: f64) {
        let api = globals::get::<BabylonApi>();
        api.fn_set_ambient_color.invoke_4(mat, r, g, b);
    }

    pub fn set_clear_color(mat: &JSObject, r: f64, g: f64, b: f64) {
        let api = globals::get::<BabylonApi>();
        api.fn_set_clear_color.invoke_4(mat, r, g, b);
    }

    pub fn set_alpha(mat: &JSObject, a: f64) {
        let api = globals::get::<BabylonApi>();
        api.fn_set_alpha.invoke_2(mat, a);
    }

    pub fn add_keyboard_observable(
        scene: &JSObject,
        callback: Box<dyn FnMut(JSValue, JSValue) -> () + Send>,
    ) {
        let cb = create_callback_2(callback);
        let api = globals::get::<BabylonApi>();
        api.fn_add_keyboard_observable.invoke_2(scene, cb);
    }

    pub fn add_observable(
        scene: &JSObject,
        observable_name: &str,
        callback: Box<dyn FnMut() -> () + Send>,
    ) {
        let cb = create_callback_0(callback);
        let api = globals::get::<BabylonApi>();
        api.fn_add_observable.invoke_3(scene, observable_name, cb);
    }

    pub fn get_delta_time(scene: &JSObject) -> f64 {
        let api = globals::get::<BabylonApi>();
        api.fn_get_delta_time.invoke_1(scene)
    }

    pub fn create_arc_rotate_camera(scene: &JSObject) -> JSObject {
        let api = globals::get::<BabylonApi>();
        api.fn_create_arc_rotate_camera
            .invoke_1(scene)
            .to_js_object()
    }

    pub fn create_hemispheric_light(scene: &JSObject) -> JSObject {
        let api = globals::get::<BabylonApi>();
        api.fn_create_hemispheric_light
            .invoke_1(scene)
            .to_js_object()
    }

    pub fn create_point_light(scene: &JSObject) -> JSObject {
        let api = globals::get::<BabylonApi>();
        api.fn_create_point_light.invoke_1(scene).to_js_object()
    }
}
