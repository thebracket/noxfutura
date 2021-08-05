use crate::render_engine::{pipeline2d, render_nf_background, GameMode, TickResult};
use egui::CtxRef;
use std::num::Wrapping;
use winit::dpi::PhysicalSize;

pub struct WorldGen {
    pipeline: Option<wgpu::RenderPipeline>,
    seed: String,
}

impl WorldGen {
    pub fn new() -> Self {
        Self {
            pipeline: None,
            seed: "Initial Test Seed".to_string(),
        }
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
                egui::Grid::new("wg_grid").show(ui, |ui| {
                    ui.label("World Seed");
                    ui.text_edit_singleline(&mut self.seed);
                    ui.end_row();
                });

                if ui.button("Generate the World").clicked() {
                    let mut seed = Wrapping::<u64>(0);
                    self.seed.chars().for_each(|n| {
                        seed += Wrapping(n as u64);
                    });

                    result = TickResult::MakePlanet { seed: seed.0 };
                }
            });

        result
    }
}
