#[macro_use]
extern crate lazy_static;
mod engine;

fn main() {
    engine::opengl::main_loop();
}