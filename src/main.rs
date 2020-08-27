use bengine::*;

struct NoxFutura {
    background_image: usize,
    quad_vert_shader: usize,
    quad_frag_shader: usize,
    quad_buffer: usize,
    quad_bg: Option<gpu::BindGroup>,
    pipeline: Option<gpu::RenderPipeline>
}

impl NoxFutura {
    fn new() -> Self {
        Self {
            background_image: 0,
            quad_frag_shader: 0,
            quad_vert_shader: 0,
            quad_buffer: 0,
            quad_bg: None,
            pipeline: None
        }
    }
}

impl BEngineGame for NoxFutura {
    fn init(&mut self, init: &mut Initializer) {
        self.background_image = init.load_texture_from_bytes(include_bytes!("../resources/images/background_image.png"));

        self.quad_vert_shader = init.load_shader_from_include(
            ShaderType::Vertex,
            gpu::include_spirv!("../resources/shaders/quad_tex.vert.spv")
        );
        self.quad_frag_shader = init.load_shader_from_include(
            ShaderType::Vertex,
            gpu::include_spirv!("../resources/shaders/quad_tex.frag.spv")
        );

        self.quad_buffer = init.make_buffer_with_data(
            &[2, 2], 
            24, 
            gpu::BufferUsage::VERTEX, 
            &[
            -1.0, 1.0, 0.0, 0.0, -1.0, -1.0, 0.0, 1.0, 1.0, -1.0, 1.0, 1.0, -1.0, 1.0, 0.0, 0.0,
            1.0, -1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.0,
            ]
        );
        let tex_layout = init.simple_texture_bg_layout("quad_layout");
        self.quad_bg = Some(init.simple_texture_bg(&tex_layout, self.background_image));
        let pipeline_layout = init.pipeline_layout(&[&tex_layout], "quad_pipeline");
        self.pipeline = Some(init.render_pipeline_simple(
            "QuadPipeline",
            &pipeline_layout,
            self.quad_vert_shader,
            self.quad_frag_shader,
            self.quad_buffer
        ));
    }

    fn tick(&mut self, core: &mut Core) -> bool {
        let mut encoder = core.device.create_command_encoder(&gpu::CommandEncoderDescriptor { label: None });
        {
            let mut rpass = encoder.begin_render_pass(&gpu::RenderPassDescriptor {
                color_attachments: &[gpu::RenderPassColorAttachmentDescriptor {
                    attachment: &core.frame.output.view,
                    resolve_target: None,
                    ops: gpu::Operations {
                        load: gpu::LoadOp::Clear(gpu::Color::BLACK),
                        store: true,
                    }
                }],
                depth_stencil_attachment: None,
            });
            rpass.set_pipeline(&self.pipeline.as_ref().unwrap());
            rpass.set_bind_group(0, &self.quad_bg.as_ref().unwrap(), &[]);
            rpass.set_vertex_buffer(
                0, 
                core.buffers.get_buffer(self.quad_buffer).buffer.as_ref().unwrap().slice(..)
            );
            rpass.draw(0..24, 0..1);
        }
        core.queue.submit(Some(encoder.finish()));

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
