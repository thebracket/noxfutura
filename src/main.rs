use bengine::*;

struct NoxFutura {
    background_image: usize,
    quad_vert_shader: usize,
    quad_frag_shader: usize,
    quad_buffer: usize
}

impl NoxFutura {
    fn new() -> Self {
        Self {
            background_image: 0,
            quad_frag_shader: 0,
            quad_vert_shader: 0,
            quad_buffer: 0
        }
    }
}

impl BEngineGame for NoxFutura {
    fn init(&mut self, init: &mut Initializer) {
        self.background_image = init.load_texture_from_bytes(include_bytes!("../resources/images/background_image.png"));

        self.quad_vert_shader = init.load_shader_from_file("resources/shaders/quad_tex.vert", ShaderType::Vertex);
        self.quad_frag_shader = init.load_shader_from_file("resources/shaders/quad_tex.frag", ShaderType::Vertex);

        self.quad_buffer = init.make_buffer_with_data(
            &[2, 2], 
            24, 
            gpu::BufferUsage::VERTEX, 
            &[
            -1.0, 1.0, 0.0, 0.0, -1.0, -1.0, 0.0, 1.0, 1.0, -1.0, 1.0, 1.0, -1.0, 1.0, 0.0, 0.0,
            1.0, -1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.0,
            ]
        );
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
