#[macro_use]
extern crate lazy_static;

mod engine;
pub mod modes;
pub mod planet;
pub mod raws;
pub mod region;
pub mod utils;
//pub mod worldmap;

mod opengl {
    pub use crate::engine::support::gl::Gl;
    pub use crate::engine::support::gl::*;
    pub use crate::engine::support::gl::types::*;
    #[macro_use]
    pub use crate::engine::glerror::*;
    pub use crate::engine::Texture;
    pub use crate::engine::Shader;
    pub use crate::engine::vertex_buffer::{VertexArray, VertexArrayEntry};
    pub use imgui::*;
}

fn main() {
    engine::main_loop();
}
