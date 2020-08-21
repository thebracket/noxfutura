#[macro_use]
extern crate lazy_static;

mod engine;
pub mod messaging;
pub mod modes;
pub mod systems;
pub mod utils;

fn main() {
    engine::main_loop();
}
