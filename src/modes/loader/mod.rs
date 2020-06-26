mod ui;
use crate::modes::playgame::*;
use parking_lot::RwLock;
use std::thread;
pub use ui::*;

lazy_static! {
    pub static ref LOADER: RwLock<LoaderState> = RwLock::new(LoaderState::new());
}

pub struct LoaderState {
    progress: f32,
    status: String,
    done: bool,

    pub rpass: Option<BlockRenderPass>,
    pub sun_render: Option<SunlightPass>,
    pub vpass: Option<VoxRenderPass>,
    pub sun_terrain: Option<SunDepthTerrainPass>,
    pub sun_vox: Option<SunDepthVoxPass>
}

impl LoaderState {
    pub fn new() -> Self {
        Self {
            progress: 0.0,
            status: "Randomly Flipping Bits...".to_string(),
            done: false,
            rpass: None,
            sun_render: None,
            vpass: None,
            sun_terrain: None,
            sun_vox: None
        }
    }

    pub fn start_loading() {
        thread::spawn(|| {
            LOADER
                .write()
                .update(0.01, "Starting to load things", false);
            crate::raws::load_raws();
            let rpass = BlockRenderPass::new();
            let vox_pass = VoxRenderPass::new(&rpass.uniform_bind_group_layout);
            let stpass = SunDepthTerrainPass::new();
            let svpass = SunDepthVoxPass::new(&vox_pass.vox_models.vertices, &vox_pass.instance_buffer);
            let sunlight_pass = SunlightPass::new(&rpass.gbuffer, &stpass.depth_view, &stpass.depth_sampler);

            let mut lock = LOADER.write();
            lock.rpass = Some(rpass);
            lock.sun_render = Some(sunlight_pass);
            lock.vpass = Some(vox_pass);
            lock.sun_terrain = Some(stpass);
            lock.sun_vox = Some(svpass);
            std::mem::drop(lock);
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
