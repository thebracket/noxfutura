use super::shader_from_bytes;
use crate::make_buffer_with_data;
use crate::FloatBuffer;
use crate::{
    pipeline_layout, render_pipeline_simple, simple_texture_bg, simple_texture_bg_layout,
    RENDER_CONTEXT,
};

pub struct BackgroundQuad {
    quad_buffer: FloatBuffer<f32>,
    pipeline: wgpu::RenderPipeline,
    tex_layout: wgpu::BindGroupLayout,
}

impl BackgroundQuad {
    pub fn new() -> Self {
        let tex_layout = simple_texture_bg_layout("quad_layout");
        let pipeline_layout = pipeline_layout(&[&tex_layout], "quad_pipeline");

        let (quad_vert_shader, quad_frag_shader) = shader_from_bytes(
            wgpu::include_spirv!("quad_tex.vert.spv"),
            wgpu::include_spirv!("quad_tex.frag.spv"),
        );

        let quad_buffer = make_buffer_with_data(
            &[2, 2],
            24,
            wgpu::BufferUsage::VERTEX,
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

        BackgroundQuad {
            quad_buffer,
            pipeline,
            tex_layout,
        }
    }

    pub fn render(&self, tex_id: usize, core: &crate::Core) {
        // Draw the background image
        let quad_bg = simple_texture_bg(&self.tex_layout, tex_id);

        let rcl = RENDER_CONTEXT.read();
        let rc = rcl.as_ref().unwrap();
        let mut encoder = rc
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                    attachment: &core.frame.output.view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });
            rpass.set_pipeline(&self.pipeline);
            rpass.set_bind_group(0, &quad_bg, &[]);
            rpass.set_vertex_buffer(0, self.quad_buffer.slice());
            rpass.draw(0..6, 0..1);
        }
        rc.queue.submit(Some(encoder.finish()));
    }
}
