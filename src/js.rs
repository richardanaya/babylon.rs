pub fn log(msg: &str) {
    crate::BabylonApi::log(msg);
}

pub fn error(msg: &str) {
    crate::BabylonApi::error(msg);
}

pub fn debugger() {
    crate::BabylonApi::debugger();
}

pub fn random() -> f32 {
    crate::BabylonApi::random()
}
