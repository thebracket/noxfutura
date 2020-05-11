#[macro_use]
extern crate lazy_static;

mod engine;
pub mod modes;
pub mod planet;
pub mod raws;
pub mod worldmap;
pub mod region;

fn main() {
    engine::main_loop();
}
