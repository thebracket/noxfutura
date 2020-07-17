#[derive(Clone)]
pub struct RenderFlags {
    pub lights_changed: bool,
    pub models_changed: bool,
    pub terrain_changed: bool,
    pub dirty_tiles: Vec<usize>
}

impl RenderFlags {
    pub fn new() -> Self {
        Self {
            lights_changed: false,
            models_changed: false,
            terrain_changed: false,
            dirty_tiles: Vec::new()
        }
    }

    pub fn reset(&mut self) {
        self.lights_changed = false;
        self.models_changed = false;
        self.terrain_changed = false;
        self.dirty_tiles.clear();
    }
}

pub fn get_render_flags() -> RenderFlags {
    use super::backends::RENDER_FLAGS;
    RENDER_FLAGS.lock().clone()
}
