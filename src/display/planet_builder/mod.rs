use crate::{
    render_engine::{pipeline2d, render_nebula_background, GameMode, TickResult},
    simulation::WorldBuilder,
};
use egui::CtxRef;
use winit::dpi::PhysicalSize;
use lazy_static::*;
use parking_lot::Mutex;

lazy_static!{
    pub static ref WORLD_GEN_STATUS : Mutex<String> = Mutex::new("Warming Up".to_string());
}
pub struct PlanetBuilder {
    pipeline: Option<wgpu::RenderPipeline>,
    seed: u64,
    thread_started: bool,
}

impl PlanetBuilder {
    pub fn new() -> Self {
        Self {
            pipeline: None,
            seed: 0,
            thread_started: false,
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
        render_nebula_background(&self.pipeline, swap_chain_texture);

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
