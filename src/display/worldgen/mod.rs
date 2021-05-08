use egui::CtxRef;
use crate::render_engine::{GameMode, TickResult, pipeline2d, render_nf_background};

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

    fn tick(&mut self, egui: &CtxRef, swap_chain_texture: &wgpu::SwapChainTexture) -> TickResult {
        render_nf_background(&self.pipeline, swap_chain_texture);

        let result = TickResult::Continue;

        egui::Window::new("World Generation Parameters")
            .auto_sized()
            .show(egui, |ui| {
                ui.label("Go here");
            }
        );

        result
    }
}
