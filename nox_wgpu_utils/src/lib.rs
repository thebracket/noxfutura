#[macro_use]
extern crate lazy_static;

use parking_lot::RwLock;
mod shader;
mod vertex_buffer;
pub use vertex_buffer::VertexBuffer;
mod context;
pub mod texture;
pub use context::Context;
pub mod pipelines;
pub mod renderpass;
pub mod uniforms;

lazy_static! {
    pub static ref DEVICE_CONTEXT: RwLock<Option<Context>> = RwLock::new(None);
}

pub fn get_window_size() -> winit::dpi::PhysicalSize<u32> {
    let lock = DEVICE_CONTEXT.read();
    lock.as_ref().unwrap().size.clone()
}

pub fn register_shader<S: ToString>(vertex_src: S, frag_src: S) -> usize {
    let mut lock = DEVICE_CONTEXT.write();
    lock.as_mut().unwrap().register_shader(vertex_src, frag_src)
}

pub fn register_texture(bytes: &[u8], label: &str) -> usize {
    let mut lock = DEVICE_CONTEXT.write();
    lock.as_mut().unwrap().register_texture(bytes, label)
}
