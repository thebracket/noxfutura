#[macro_use]
extern crate lazy_static;

pub mod components;
mod engine;
pub mod modes;
pub mod planet;
pub mod raws;
pub mod utils;
pub mod systems;

fn main() {
    engine::main_loop();
}
