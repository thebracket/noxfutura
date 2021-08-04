use crate::render_engine::{pipeline2d, render_nf_background, GameMode, TickResult};
use egui::{Color32, CtxRef, Pos2};
mod tagline;
use tagline::tagline;
use winit::dpi::PhysicalSize;

const DEDICATION: &str =
    "To Kylah of the West and Jakie Monster: The Bravest Little Warriors of Them All.";
const COPYRIGHT: &str = "(c) 2015-2020 Bracket Productions, All Rights Reserved.";

pub struct MainMenu {
    pipeline: Option<wgpu::RenderPipeline>,
    tagline: String,
}

impl MainMenu {
    pub fn new() -> Self {
        Self {
            pipeline: None,
            tagline: tagline(),
        }
    }
}

impl GameMode for MainMenu {
    fn pre_init(&mut self) {
        self.pipeline = Some(pipeline2d());
    }

    fn tick(
        &mut self,
        size: PhysicalSize<u32>,
        egui: &CtxRef,
        swap_chain_texture: &wgpu::SwapChainTexture,
    ) -> TickResult {
        render_nf_background(&self.pipeline, swap_chain_texture);

        let mut result = TickResult::Continue;

        let center_x = (size.width / 2) as f32;
        let center_y = (size.height / 2) as f32;

        let char_width =
            egui.fonts().glyph_width(egui::TextStyle::Body, 'A') * egui.fonts().pixels_per_point();
        let tag_width = self.tagline.len() as f32 * char_width;
        let dedication_width = DEDICATION.len() as f32 * char_width;

        egui::Window::new("Tagline")
            .title_bar(false)
            .auto_sized()
            .resizable(false)
            .fixed_pos(Pos2::new(center_x - (tag_width / 2.0), center_y + 150.0))
            .show(egui, |ui| {
                ui.colored_label(Color32::from_rgb(255, 0, 0), self.tagline.clone());
            });

        egui::Window::new("Nox Futura")
            .auto_sized()
            .resizable(false)
            .title_bar(false)
            .fixed_pos(Pos2::new(
                center_x - ((char_width * 24.0) / 2.0),
                center_y - 180.0,
            ))
            .show(egui, |ui| {
                if ui.button("Create World").clicked() {
                    result = TickResult::WorldGen;
                }

                if ui.button("Quit").clicked() {
                    result = TickResult::Quit;
                }
            });

        egui::Window::new("Dedication")
            .auto_sized()
            .resizable(false)
            .title_bar(false)
            .fixed_pos(Pos2::new(
                center_x - (dedication_width / 2.0),
                center_y + 180.0,
            ))
            .show(egui, |ui| {
                ui.colored_label(Color32::from_rgb(255, 255, 255), DEDICATION);
            });

        egui::Window::new("Copyright")
            .title_bar(false)
            .auto_sized()
            .resizable(false)
            .fixed_pos(Pos2::new(1.0, size.height as f32 - 25.0))
            .show(egui, |ui| {
                ui.colored_label(Color32::from_rgb(255, 255, 0), COPYRIGHT);
            });

        result
    }
}
