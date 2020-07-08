#[derive(Copy, Clone)]
pub struct RenderFlags {
    pub lights_changed : bool,
    pub models_changed : bool,
    pub terrain_changed : bool
}

impl RenderFlags {
    pub fn new() -> Self {
        Self {
            lights_changed : false,
            models_changed : false,
            terrain_changed : false
        }
    }

    pub fn reset(&mut self) {
        self.lights_changed = false;
        self.models_changed = false;
        self.terrain_changed = false;
    }
}

pub fn get_render_flags() -> renderflags::RenderFlags {
    use super::backends::RENDER_FLAGS;
    RENDER_FLAGS.lock().clone()
}