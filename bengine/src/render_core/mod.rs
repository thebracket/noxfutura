mod init;
mod render_context;

pub(crate) use init::initialize_imgui;
pub(crate) use render_context::init_render_context;
pub use render_context::RENDER_CONTEXT;
