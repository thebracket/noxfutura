use crate::simulation::{WORLD_HEIGHT, WORLD_WIDTH};
use crate::{
    render_engine::{
        pipeline2d, render_fullscreen_texture, render_nebula_background, GameMode, Texture,
        TickResult,
    },
    simulation::WorldBuilder,
};
use egui::CtxRef;
use lazy_static::*;
use parking_lot::Mutex;
use winit::dpi::PhysicalSize;

pub enum WorldGenDisplayMode {
    Erosion,
}

pub struct WorldGenDisplay {
    pub mode: WorldGenDisplayMode,
    pub dirty: bool,
    pub base_altitude: Vec<u8>,
}

lazy_static! {
    pub static ref WORLD_GEN_STATUS: Mutex<String> = Mutex::new("Warming Up".to_string());
}

lazy_static! {
    pub static ref WORLD_GEN_DISPLAY: Mutex<WorldGenDisplay> = Mutex::new(WorldGenDisplay {
        mode: WorldGenDisplayMode::Erosion,
        dirty: true,
        base_altitude: vec![0u8; 256 * 256]
    });
}

pub struct PlanetBuilder {
    pipeline: Option<wgpu::RenderPipeline>,
    seed: u64,
    thread_started: bool,
    wg_map: Option<Texture>,
}

impl PlanetBuilder {
    pub fn new() -> Self {
        Self {
            pipeline: None,
            seed: 0,
            thread_started: false,
            wg_map: None,
        }
    }
}

impl GameMode for PlanetBuilder {
    fn pre_init(&mut self) {
        self.pipeline = Some(pipeline2d());
    }

    fn world_gen_params(&mut self, seed: u64) {
        self.seed = seed;
    }

    fn tick(
        &mut self,
        _size: PhysicalSize<u32>,
        egui: &CtxRef,
        swap_chain_texture: &wgpu::SwapChainTexture,
    ) -> TickResult {
        {
            let mut wg_display_lock = WORLD_GEN_DISPLAY.lock();
            if wg_display_lock.dirty {
                // Build the texture
                let mut image_rgba = vec![0u8; WORLD_WIDTH * WORLD_HEIGHT * 4];
                wg_display_lock
                    .base_altitude
                    .iter()
                    .enumerate()
                    .for_each(|(idx, h)| {
                        if *h == 0 {
                            image_rgba[(idx * 4) + 2] = 128;
                        } else {
                            image_rgba[(idx * 4) + 1] = *h;
                        }
                    });
                let tex = crate::render_engine::memory_texture(
                    "wgmap".to_string(),
                    256,
                    256,
                    &image_rgba,
                );
                self.wg_map = Some(tex);
                wg_display_lock.dirty = false;
            }
        }

        if let Some(wg_map) = &self.wg_map {
            render_fullscreen_texture(&self.pipeline, swap_chain_texture, &wg_map);
        } else {
            render_nebula_background(&self.pipeline, swap_chain_texture);
        }

        let result = TickResult::Continue;

        egui::Window::new("World Generation Progress").show(egui, |ui| {
            ui.label(&WORLD_GEN_STATUS.lock().clone());
        });

        if !self.thread_started {
            self.thread_started = true;
            let seed = self.seed.to_owned();
            std::thread::spawn(move || {
                let mut builder = WorldBuilder::new(seed);
                builder.go();
            });
        }

        result
    }
}
