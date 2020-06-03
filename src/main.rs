#[macro_use]
extern crate lazy_static;

mod engine;
pub mod modes;
pub mod planet;
pub mod raws;
pub mod utils;
pub mod components;

fn main() {
    engine::main_loop();
}
