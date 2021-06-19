use crate::render_engine::{pipeline2d, render_nebula_background, GameMode, TickResult};
use egui::CtxRef;
use winit::dpi::PhysicalSize;

pub struct PlanetBuilder {
    pipeline: Option<wgpu::RenderPipeline>,
}

impl PlanetBuilder {
    pub fn new() -> Self {
        Self { pipeline: None }
    }
}

impl GameMode for PlanetBuilder {
    fn pre_init(&mut self) {
        self.pipeline = Some(pipeline2d());
    }

    fn tick(
        &mut self,
        _size: PhysicalSize<u32>,
        egui: &CtxRef,
        swap_chain_texture: &wgpu::SwapChainTexture,
    ) -> TickResult {
        render_nebula_background(&self.pipeline, swap_chain_texture);

        let result = TickResult::Continue;

        egui::Window::new("World Generation Information")
            .auto_sized()
            .show(egui, |ui| {
                ui.label("Go here");
            });

        result
    }
}
