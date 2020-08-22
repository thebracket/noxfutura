use bengine::*;

struct NoxFutura {
    background_image: usize,
    quad_vert_shader: usize,
    quad_frag_shader: usize
}

impl NoxFutura {
    fn new() -> Self {
        Self {
            background_image: 0,
            quad_frag_shader: 0,
            quad_vert_shader: 0
        }
    }
}

impl BEngineGame for NoxFutura {
    fn init(&mut self, device: &gpu::Device, queue: &gpu::Queue, textures: &mut Textures, shaders: &mut Shaders) {
        self.background_image = textures.load_texture_from_bytes(
            device,
            queue,
            include_bytes!("../resources/images/background_image.png"),
            "Background"
        );
        self.quad_vert_shader = shaders.register("resources/shaders/quad_tex.vert", ShaderType::Vertex, device);
        self.quad_frag_shader = shaders.register("resources/shaders/quad_tex.frag", ShaderType::Vertex, device);
    }

    fn tick(&mut self, core: &mut Core) -> bool {
        let thanks = gui::Window::new(gui::im_str!("Thanks to our supporters"));
        thanks
            .position([300.0, 125.0], gui::Condition::Always)
            .size([400.0, 400.0], gui::Condition::FirstUseEver)
            .always_auto_resize(true)
            .collapsible(false)
            .build(core.imgui, || {
                core.imgui.text(gui::im_str!("Noah Bogart via Patreon"));
            });

        true
    }
}

fn main() {
    run(NoxFutura::new() );
}
