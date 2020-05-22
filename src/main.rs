#[macro_use]
extern crate lazy_static;

mod engine;
pub mod modes;
pub mod planet;
pub mod raws;
pub mod region;
pub mod utils;
mod render_engine;
mod gameplay;

mod opengl {
    pub use crate::engine::support::gl::Gl;
    pub use crate::engine::support::gl::*;
    pub use crate::engine::support::gl::types::*;
    pub use crate::engine::glerror::*;
    pub use crate::engine::Texture;
    pub use crate::engine::Shader;
    pub use crate::engine::vertex_buffer::{VertexArray, VertexArrayEntry};
    pub use imgui::*;
    pub use crate::engine::EngineContext;
}

pub mod prelude {
    pub use crate::planet::*;
    pub use crate::region::*;
    pub use legion::prelude::*;
    pub use crate::gameplay::*;
    pub use crate::render_engine::*;
}

fn main() {
    engine::main_loop();
}
