pub fn log(msg: &str) {
    crate::api::BabylonApi::log(msg);
}

pub fn error(msg: &str) {
    crate::api::BabylonApi::error(msg);
}

pub fn debugger() {
    crate::api::BabylonApi::debugger();
}

pub fn random() -> f64 {
    crate::api::BabylonApi::random()
}
