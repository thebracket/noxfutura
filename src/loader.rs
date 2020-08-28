use crate::{GameMode, NoxMode};
use bengine::*;

pub struct Loader {
    quad_buffer: FloatBuffer<f32>,
    quad_bg: gpu::BindGroup,
    pipeline: gpu::RenderPipeline,
}

impl Loader {
    pub fn new() -> Self {
        use helpers::*;
        let background_image = texture_from_file("resources/images/background_image.png", "nox_bg");
        let tex_layout = simple_texture_bg_layout("quad_layout");
        let pipeline_layout = pipeline_layout(&[&tex_layout], "quad_pipeline");

        let (quad_vert_shader, quad_frag_shader) = shader_from_bytes(
            gpu::include_spirv!("../resources/shaders/quad_tex.vert.spv"),
            gpu::include_spirv!("../resources/shaders/quad_tex.frag.spv"),
        );

        let quad_buffer = make_buffer_with_data(
            &[2, 2],
            24,
            gpu::BufferUsage::VERTEX,
            &[
                -1.0, 1.0, 0.0, 0.0, -1.0, -1.0, 0.0, 1.0, 1.0, -1.0, 1.0, 1.0, -1.0, 1.0, 0.0,
                0.0, 1.0, -1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.0,
            ],
        );
        let pipeline = render_pipeline_simple(
            "QuadPipeline",
            &pipeline_layout,
            quad_vert_shader,
            quad_frag_shader,
            &quad_buffer,
        );
        Self {
            quad_buffer,
            quad_bg: simple_texture_bg(&tex_layout, background_image),
            pipeline,
        }
    }
}

impl NoxMode for Loader {
    fn tick(&mut self, core: &mut Core) -> GameMode {
        // Draw the background image
        let rcl = RENDER_CONTEXT.read();
        let rc = rcl.as_ref().unwrap();
        let mut encoder = rc
            .device
            .create_command_encoder(&gpu::CommandEncoderDescriptor { label: None });
        {
            let mut rpass = encoder.begin_render_pass(&gpu::RenderPassDescriptor {
                color_attachments: &[gpu::RenderPassColorAttachmentDescriptor {
                    attachment: &core.frame.output.view,
                    resolve_target: None,
                    ops: gpu::Operations {
                        load: gpu::LoadOp::Clear(gpu::Color::BLACK),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });
            rpass.set_pipeline(&self.pipeline);
            rpass.set_bind_group(0, &self.quad_bg, &[]);
            rpass.set_vertex_buffer(0, self.quad_buffer.slice());
            rpass.draw(0..24, 0..1);
        }
        rc.queue.submit(Some(encoder.finish()));

        GameMode::Loader
    }
}
