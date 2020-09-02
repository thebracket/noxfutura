use crate::planet::PlanetParams;
use crate::{GameMode, NoxMode, SharedResources};
use bengine::*;
use bracket_random::prelude::RandomNumberGenerator;

pub struct WorldGen1 {
    params: PlanetParams,
}

impl WorldGen1 {
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
}

impl NoxMode for WorldGen1 {
    fn tick(&mut self, core: &mut Core, shared: &SharedResources) -> GameMode {
        use gui::*;

        let mut result = GameMode::WorldGen1;
        shared.quad_render.render(shared.background_image, core);

        let window = Window::new(im_str!("World Generation Parameters"));
        window.always_auto_resize(true).build(core.imgui, || {
            core.imgui
                .input_int(im_str!("World Seed"), &mut self.params.world_seed)
                .build();
            Slider::new(im_str!("Water Level"), 1..=4)
                .build(core.imgui, &mut self.params.water_level);
            Slider::new(im_str!("Plains Level"), 1..=4)
                .build(core.imgui, &mut self.params.plains_level);
            Slider::new(im_str!("Starting Settlers"), 1..=20)
                .build(core.imgui, &mut self.params.starting_settlers);
            core.imgui.checkbox(
                im_str!("Require Teleport Beacon"),
                &mut self.params.strict_beamdown,
            );
            core.imgui
                .checkbox(im_str!("Extra Noise Level"), &mut self.params.extra_noise);
            if core.imgui.button(im_str!("Build Planet"), [400.0, 50.0]) {
                crate::planet::start_building_planet(self.params.clone());
                result = GameMode::WorldGen2;
            }
        });

        result
    }
}
