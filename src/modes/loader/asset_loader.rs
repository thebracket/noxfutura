use super::super::{GBuffer, Palette};
use parking_lot::RwLock;
use std::thread;

lazy_static! {
    pub static ref LOADER: RwLock<LoaderState> = RwLock::new(LoaderState::new());
}

pub struct LoaderState {
    pub progress: f32,
    pub status: String,
    pub done: bool,
    pub palette: Option<Palette>,
    pub tex_array: Option<crate::modes::TextureArray>,
    pub g_buffer: Option<GBuffer>,
}

impl LoaderState {
    pub fn new() -> Self {
        Self {
            progress: 0.0,
            status: "Randomly Flipping Bits...".to_string(),
            done: false,
            tex_array: None,
            g_buffer: None,
            palette: None,
        }
    }

    pub fn start_loading() {
        thread::spawn(|| {
            LOADER.write().update(0.01, "Loading Raw Files", false);

            crate::load_raws();

            LOADER.write().update(0.02, "Fingerpainting", false);
            let palette = super::super::Palette::new();
            LOADER.write().palette = Some(palette);

            LOADER.write().update(0.03, "Baking Materials", false);
            let tex_array = super::super::TextureArray::blank().expect("Unable to load textures");
            LOADER.write().tex_array = Some(tex_array);

            let gbuffer = GBuffer::new();
            LOADER.write().g_buffer = Some(gbuffer);

            LOADER.write().update(1.00, "Built all the things", true);
        });
    }

    fn update(&mut self, progress: f32, status: &str, is_done: bool) {
        self.progress = progress;
        self.status = status.to_string();
        self.done = is_done;
    }
}

pub fn loader_progress(progress: f32, status: &str, is_done: bool) {
    LOADER.write().update(progress, status, is_done);
}
