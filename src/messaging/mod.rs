mod backends;
mod renderflags;
use backends::*;

pub use renderflags::get_render_flags;

pub fn reset() {
    RENDER_FLAGS.lock().reset();
}

pub fn vox_moved() {
    RENDER_FLAGS.lock().models_changed = true;
}