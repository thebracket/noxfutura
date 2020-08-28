use bengine::*;
use crate::GameMode;

pub struct Loader {
    quad_buffer: usize,
    quad_bg: gpu::BindGroup,
    pipeline: gpu::RenderPipeline
}

impl Loader {
    pub fn new(init: &mut Initializer) -> Self {
        println!("Start loader init");
        let background_image = TEXTURES.write().load_texture_from_bytes(include_bytes!("../resources/images/background_image.png"), "nox_bg");
        println!("tex");
        let tex_layout = init.simple_texture_bg_layout("quad_layout");
        println!("texl");
        let pipeline_layout = init.pipeline_layout(&[&tex_layout], "quad_pipeline");
        println!("pl");
        let quad_vert_shader = init.load_shader_from_include(
            gpu::include_spirv!("../resources/shaders/quad_tex.vert.spv")
        );
        let quad_frag_shader = init.load_shader_from_include(
            gpu::include_spirv!("../resources/shaders/quad_tex.frag.spv")
        );
        println!("shaders");
        let quad_buffer = init.make_buffer_with_data(
            &[2, 2],
            24,
            gpu::BufferUsage::VERTEX,
            &[
            -1.0, 1.0, 0.0, 0.0, -1.0, -1.0, 0.0, 1.0, 1.0, -1.0, 1.0, 1.0, -1.0, 1.0, 0.0, 0.0,
            1.0, -1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.0,
            ]
        );
        println!("ProgInit");
        Self {
            quad_buffer,
            quad_bg: init.simple_texture_bg(&tex_layout, background_image),
            pipeline: init.render_pipeline_simple(
                "QuadPipeline",
                &pipeline_layout,
                quad_vert_shader,
                quad_frag_shader,
                quad_buffer
            )
        }
    }

    pub fn render(&mut self, core: &mut Core) -> GameMode {
        // Draw the background image
        let rcl = RENDER_CONTEXT.read();
        let rc = rcl.as_ref().unwrap();
        let mut encoder = rc.device.create_command_encoder(&gpu::CommandEncoderDescriptor { label: None });
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
            rpass.set_pipeline(&self.pipeline);
            rpass.set_bind_group(0, &self.quad_bg, &[]);
            rpass.set_vertex_buffer(
                0, 
                core.buffers.get_buffer(self.quad_buffer).buffer.as_ref().unwrap().slice(..)
            );
            rpass.draw(0..24, 0..1);
        }
        rc.queue.submit(Some(encoder.finish()));

        GameMode::Loader
    }
}