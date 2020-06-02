use super::resources::SharedResources;
use crate::planet::PlanetParams;
use bracket_random::prelude::*;
use imgui::*;

pub struct PlanetGen {
    params: crate::planet::PlanetParams,
}

impl PlanetGen {
    pub fn new() -> Self {
        let mut rng = RandomNumberGenerator::new();
        Self {
            params: PlanetParams {
                world_seed: rng.range(-2147483648, 2147483647),
                water_level: 3,
                plains_level: 3,
                starting_settlers: 6,
                strict_beamdown: true,
                extra_noise: true,
            },
        }
    }

    pub fn tick(
        &mut self,
        resources: &SharedResources,
        frame: &wgpu::SwapChainOutput,
        context: &mut crate::engine::Context,
        ui: &imgui::Ui,
    ) -> super::ProgramMode {
        let mut result = super::ProgramMode::PlanetGen;
        super::helpers::render_menu_background(context, frame, resources);

        let window = imgui::Window::new(im_str!("World Generation Parameters"));
        window.always_auto_resize(true).build(ui, || {
            ui.input_int(im_str!("World Seed"), &mut self.params.world_seed)
                .build();
            Slider::new(im_str!("Water Level"), 1..=4).build(ui, &mut self.params.water_level);
            Slider::new(im_str!("Plains Level"), 1..=4).build(ui, &mut self.params.plains_level);
            Slider::new(im_str!("Starting Settlers"), 1..=20)
                .build(ui, &mut self.params.starting_settlers);
            ui.checkbox(
                im_str!("Require Teleport Beacon"),
                &mut self.params.strict_beamdown,
            );
            ui.checkbox(im_str!("Extra Noise Level"), &mut self.params.extra_noise);
            if ui.button(im_str!("Build Planet"), [400.0, 50.0]) {
                crate::planet::start_building_planet(self.params.clone());
                result = super::ProgramMode::PlanetGen2;
            }
        });

        result
    }
}
