use super::renderflags::RenderFlags;
use parking_lot::Mutex;

lazy_static! {
    pub static ref RENDER_FLAGS: Mutex<RenderFlags> = Mutex::new(RenderFlags::new());
}
