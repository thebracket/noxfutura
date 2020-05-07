#[macro_use]
extern crate lazy_static;

mod engine;
pub mod modes;
pub mod planet;
pub mod worldmap;
pub mod raws;

fn main() {
    engine::main_loop();
}
