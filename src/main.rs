#[macro_use]
extern crate lazy_static;

mod engine;
pub mod modes;
pub mod systems;
pub mod utils;
pub mod messaging;

fn main() {
    engine::main_loop();
}
