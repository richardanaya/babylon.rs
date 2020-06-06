use babylon::*;

#[no_mangle]
pub fn main() -> () {
    js!(console.log).invoke_1("Hello World");
}