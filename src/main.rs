#[macro_use]
extern crate lazy_static;

mod engine;
pub mod modes;
pub mod worldmap;
pub mod planet;

fn main() {
    engine::main_loop();
}
