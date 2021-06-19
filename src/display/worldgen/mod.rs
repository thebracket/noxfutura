use crate::render_engine::{pipeline2d, render_nf_background, GameMode, TickResult};
use egui::CtxRef;
use winit::dpi::PhysicalSize;

pub struct WorldGen {
    pipeline: Option<wgpu::RenderPipeline>,
}

impl WorldGen {
    pub fn new() -> Self {
        Self { pipeline: None }
    }
}

impl GameMode for WorldGen {
    fn pre_init(&mut self) {
        self.pipeline = Some(pipeline2d());
    }

    fn tick(
        &mut self,
        _size: PhysicalSize<u32>,
        egui: &CtxRef,
        swap_chain_texture: &wgpu::SwapChainTexture,
    ) -> TickResult {
        render_nf_background(&self.pipeline, swap_chain_texture);

        let mut result = TickResult::Continue;

        egui::Window::new("World Generation Parameters")
            .auto_sized()
            .show(egui, |ui| {
                ui.label("Go here");
                if ui.button("Generate the World").clicked() {
                    result = TickResult::MakePlanet;
                }
            });

        result
    }
}
