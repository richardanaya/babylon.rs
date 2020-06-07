use alloc::boxed::Box;

pub fn log(msg: &str) {
    crate::api::BabylonApi::log(msg);
}

pub fn error(msg: &str) {
    crate::api::BabylonApi::error(msg);
}

pub fn debugger() {
    crate::api::BabylonApi::debugger();
}

pub fn random() -> f32 {
    crate::api::BabylonApi::random()
}

pub fn set_interval(
    callback: Box<dyn FnMut() -> () + Send>,
    milliseconds: u32
) {
    let timer_api = web_timer::Timer::default();
    timer_api.set_interval(callback,milliseconds);
}