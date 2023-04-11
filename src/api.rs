use alloc::boxed::Box;
use web::*;

pub struct BabylonApi {
    fn_log: JSFunction,
    fn_error: JSFunction,
    fn_debug: JSFunction,
    fn_random: JSFunction,
    fn_create_basic_scene: JSFunction,
    fn_create_scene: JSFunction,
    fn_create_sphere: JSFunction,
    fn_create_cube: JSFunction,
    fn_create_standard_material: JSFunction,
    fn_dispose_mesh: JSFunction,
    fn_set_position: JSFunction,
    fn_set_scaling: JSFunction,
    fn_set_material: JSFunction,
    fn_set_emmisive_color: JSFunction,
    fn_set_diffuse_color: JSFunction,
    fn_set_specular_color: JSFunction,
    fn_set_ambient_color: JSFunction,
    fn_set_clear_color: JSFunction,
    fn_set_alpha: JSFunction,
    fn_add_keyboard_observable: JSFunction,
    fn_add_observable: JSFunction,
    fn_get_delta_time: JSFunction,
    fn_create_arc_rotate_camera: JSFunction,
    fn_create_hemispheric_light: JSFunction,
    fn_create_point_light: JSFunction,
    fn_create_gltf: JSFunction,
}

impl Default for BabylonApi {
    fn default() -> Self {
        BabylonApi {
            fn_create_basic_scene: js!(
                r#"
                function(selector){
                    var canvas = document.querySelector(selector);
                    var engine = new BABYLON.Engine(canvas, true); 
                    var createScene = function () {
                        var scene = new BABYLON.Scene(engine);
                        scene.createDefaultEnvironment({
                            createGround: false,
                            createSkybox: false,
                        });
    
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
            "#
            ),
            fn_create_scene: js!(
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
            "#
            ),
            fn_create_sphere: js!(
                r#"
                function(scene,size){
                    return BABYLON.MeshBuilder.CreateSphere(
                        null,
                        { diameter: size },
                        scene);
                }
            "#
            ),
            fn_create_cube: js!(
                r#"
                function(scene,w,h,d){
                    return BABYLON.MeshBuilder.CreateBox(
                        null,
                        { height: h, width: w, depth: d },
                        scene);
                }
            "#
            ),
            fn_create_standard_material: js!(
                r#"
                function(scene){
                    return new BABYLON.StandardMaterial(null, scene);
                }
            "#
            ),
            fn_dispose_mesh: js!(
                r#"
                function(mesh){
                    mesh.dispose()
                }
            "#
            ),
            fn_log: js!(
                r#"
                function(msg){
                    console.log(msg);
                }
            "#
            ),
            fn_error: js!(
                r#"
                function(msg){
                    console.error(msg);
                }
            "#
            ),
            fn_debug: js!(
                r#"
                function(){
                    debugger;
                }
            "#
            ),
            fn_random: js!(
                r#"
                function(){
                    return Math.random();
                }
            "#
            ),
            fn_set_position: js!(
                r#"
                function(mesh,x,y,z){
                    mesh.position = new BABYLON.Vector3(x,y,z);
                }
            "#
            ),
            fn_set_scaling: js!(
                r#"
                function(mesh,x,y,z){
                    mesh.scaling = new BABYLON.Vector3(x,y,z);
                }
            "#
            ),
            fn_set_material: js!(
                r#"
                function(mesh,mat){
                    mesh.material = mat;
                }
            "#
            ),
            fn_set_emmisive_color: js!(
                r#"
                function(mat,r,g,b){
                    mat.emissiveColor = new BABYLON.Color3(r, g, b);
                }
            "#
            ),
            fn_set_diffuse_color: js!(
                r#"
                function(mat,r,g,b){
                    mat.diffuseColor = new BABYLON.Color3(r, g, b);
                }
            "#
            ),
            fn_set_specular_color: js!(
                r#"
                function(mat,r,g,b){
                    mat.specularColor = new BABYLON.Color3(r, g, b);
                }
            "#
            ),
            fn_set_ambient_color: js!(
                r#"
                function(mat,r,g,b){
                    mat.ambientColor = new BABYLON.Color3(r, g, b);
                }
            "#
            ),
            fn_set_clear_color: js!(
                r#"
                function(scene,r,g,b){
                    scene.clearColor = new BABYLON.Color3(r, g, b);
                }
            "#
            ),
            fn_set_alpha: js!(
                r#"
                function(mat,a){
                    mat.alpha = a;
                }
            "#
            ),
            fn_add_keyboard_observable: js!(
                r#"
                function(scene,cb){
                    scene.onKeyboardObservable.add((kbInfo) => {
                        cb(kbInfo.type,kbInfo.event.keyCode)
                    });
                }
            "#
            ),
            fn_add_observable: js!(
                r#"
                function(scene,name,cb){
                    scene[name].add(() => {
                        cb()
                    });
                }
            "#
            ),
            fn_get_delta_time: js!(
                r#"
                function(scene){
                    return scene.getEngine().getDeltaTime();
                }
            "#
            ),
            fn_create_arc_rotate_camera: js!(
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
            "#
            ),
            fn_create_hemispheric_light: js!(
                r#"
                function(scene){
                    var light = new BABYLON.HemisphericLight(
                        null,
                        new BABYLON.Vector3(1, 1, 0),
                        scene
                    );
                    return light;
                }
            "#
            ),
            fn_create_point_light: js!(
                r#"
                function(scene){
                    var light = new BABYLON.PointLight(
                        null,
                        new BABYLON.Vector3(0, 1, -1),
                        scene
                    );
                    return light;
                }
            "#
            ),
            fn_create_gltf: js!(
                r#"
                function(scene,file){
                    var dummy = new BABYLON.Mesh(null, scene);
                    BABYLON.SceneLoader.ImportMesh(null, "", file, scene, function (newMeshes, particleSystems, skeletons) {
                        for(let v in newMeshes){
                            var dude = newMeshes[v];
                            dude.parent = dummy;
                        }
                    });
                    return dummy;
                }
                "#
            ),
        }
    }
}

impl BabylonApi {
    pub fn create_basic_scene(selector: &str) -> ExternRef {
        let api = globals::get::<BabylonApi>();
        api.fn_create_basic_scene.invoke_and_return_object(&[InvokeParam::String(selector)])
    }
    pub fn create_scene(selector: &str) -> ExternRef {
        let api = globals::get::<BabylonApi>();
        api.fn_create_scene.invoke_and_return_object(&[InvokeParam::String(selector)])
    }
    pub fn create_sphere(scene_ref: &ExternRef, size: f64) -> ExternRef {
        let api = globals::get::<BabylonApi>();
        api.fn_create_sphere
            .invoke_and_return_object(&[InvokeParam::ExternRef(scene_ref), InvokeParam::Float64(size)])
    }

    pub fn create_cube(scene_ref: &ExternRef, width: f64, height: f64, depth: f64) -> ExternRef {
        let api = globals::get::<BabylonApi>();
        api.fn_create_cube
            .invoke_and_return_object(&[InvokeParam::ExternRef(scene_ref), InvokeParam::Float64(width), InvokeParam::Float64(height), InvokeParam::Float64(depth)])
    }

    pub fn create_standard_material(scene_ref: &ExternRef) -> ExternRef {
        let api = globals::get::<BabylonApi>();
        api.fn_create_standard_material
            .invoke_and_return_object(&[InvokeParam::ExternRef(scene_ref)])
    }

    pub fn dispose_mesh(mesh: &ExternRef) {
        let api = globals::get::<BabylonApi>();
        api.fn_dispose_mesh.invoke(&[InvokeParam::ExternRef(mesh)]);
    }

    pub fn log(msg: &str) {
        let api = globals::get::<BabylonApi>();
        api.fn_log.invoke(&[InvokeParam::String(msg)]);
    }

    pub fn error(msg: &str) {
        let api = globals::get::<BabylonApi>();
        api.fn_error.invoke(&[InvokeParam::String(msg)]);
    }

    pub fn debugger() {
        let api = globals::get::<BabylonApi>();
        api.fn_debug.invoke(&[]);
    }

    pub fn random() -> f64 {
        let api = globals::get::<BabylonApi>();
        api.fn_random.invoke(&[])
    }

    pub fn set_position(mesh: &ExternRef, x: f64, y: f64, z: f64) {
        let api = globals::get::<BabylonApi>();
        api.fn_set_position.invoke(&[InvokeParam::ExternRef(mesh), InvokeParam::Float64(x), InvokeParam::Float64(y), InvokeParam::Float64(z)]);
    }

    pub fn set_scaling(mesh: &ExternRef, x: f64, y: f64, z: f64) {
        let api = globals::get::<BabylonApi>();
        api.fn_set_scaling.invoke(&[InvokeParam::ExternRef(mesh), InvokeParam::Float64(x), InvokeParam::Float64(y), InvokeParam::Float64(z)]);
    }

    pub fn set_material(mesh: &ExternRef, mat: &ExternRef) {
        let api = globals::get::<BabylonApi>();
        api.fn_set_material.invoke(&[InvokeParam::ExternRef(mesh), InvokeParam::ExternRef(mat)]);
    }

    pub fn set_emmisive_color(mat: &ExternRef, r: f64, g: f64, b: f64) {
        let api = globals::get::<BabylonApi>();
        api.fn_set_emmisive_color.invoke(&[InvokeParam::ExternRef(mat), InvokeParam::Float64(r), InvokeParam::Float64(g), InvokeParam::Float64(b)]);
    }

    pub fn set_diffuse_color(mat: &ExternRef, r: f64, g: f64, b: f64) {
        let api = globals::get::<BabylonApi>();
        api.fn_set_diffuse_color.invoke(&[InvokeParam::ExternRef(mat), InvokeParam::Float64(r), InvokeParam::Float64(g), InvokeParam::Float64(b)]);
    }

    pub fn set_specular_color(mat: &ExternRef, r: f64, g: f64, b: f64) {
        let api = globals::get::<BabylonApi>();
        api.fn_set_specular_color.invoke(&[InvokeParam::ExternRef(mat), InvokeParam::Float64(r), InvokeParam::Float64(g), InvokeParam::Float64(b)]);
    }

    pub fn set_ambient_color(mat: &ExternRef, r: f64, g: f64, b: f64) {
        let api = globals::get::<BabylonApi>();
        api.fn_set_ambient_color.invoke(&[InvokeParam::ExternRef(mat), InvokeParam::Float64(r), InvokeParam::Float64(g), InvokeParam::Float64(b)]);
    }

    pub fn set_clear_color(mat: &ExternRef, r: f64, g: f64, b: f64) {
        let api = globals::get::<BabylonApi>();
        api.fn_set_clear_color.invoke(&[InvokeParam::ExternRef(mat), InvokeParam::Float64(r), InvokeParam::Float64(g), InvokeParam::Float64(b)]);
    }

    pub fn set_alpha(mat: &ExternRef, a: f64) {
        let api = globals::get::<BabylonApi>();
        api.fn_set_alpha.invoke(&[InvokeParam::ExternRef(mat), InvokeParam::Float64(a)]);
    }

    pub fn add_keyboard_observable(
        scene: &ExternRef,
        callback: Box<dyn FnMut(f64, f64) -> () + Send>,
    ) {
        /*let cb = callback;
        let api = globals::get::<BabylonApi>();
        api.fn_add_keyboard_observable.invoke(&[InvokeParam::ExternRef(scene), InvokeParam::ExternRef(cb)]);*/
    }

    pub fn add_observable(
        scene: &ExternRef,
        observable_name: &str,
        callback: Box<dyn FnMut() -> () + Send>,
    ) {
        /*let cb = create_callback(callback);
        let api = globals::get::<BabylonApi>();
        api.fn_add_observable.invoke(&[InvokeParam::ExternRef(scene), InvokeParam::String(observable_name),  cb]);*/
    }

    pub fn get_delta_time(scene: &ExternRef) -> f64 {
        let api = globals::get::<BabylonApi>();
        api.fn_get_delta_time.invoke(&[InvokeParam::ExternRef(scene)])
    }

    pub fn create_arc_rotate_camera(scene: &ExternRef) -> ExternRef {
        let api = globals::get::<BabylonApi>();
        api.fn_create_arc_rotate_camera
            .invoke_and_return_object(&[InvokeParam::ExternRef(scene)])
    }

    pub fn create_hemispheric_light(scene: &ExternRef) -> ExternRef {
        let api = globals::get::<BabylonApi>();
        api.fn_create_hemispheric_light
            .invoke_and_return_object(&[InvokeParam::ExternRef(scene)])
    }

    pub fn create_point_light(scene: &ExternRef) -> ExternRef {
        let api = globals::get::<BabylonApi>();
        api.fn_create_point_light.invoke_and_return_object(&[InvokeParam::ExternRef(scene)])
    }

    pub fn create_gltf(scene: &ExternRef, file: &str) -> ExternRef {
        let api = globals::get::<BabylonApi>();
        api.fn_create_gltf
            .invoke_and_return_object(&[InvokeParam::ExternRef(scene), InvokeParam::String(file)])
    }
}
