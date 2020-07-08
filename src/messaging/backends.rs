use parking_lot::Mutex;
use super::renderflags::RenderFlags;

lazy_static! {
    pub static ref RENDER_FLAGS: Mutex<RenderFlags> = Mutex::new(RenderFlags::new());
}
