use super::{Initializer, Core};
use crate::ShaderType;
use std::collections::HashMap;

pub struct BackgroundRender {
    quad_vert_shader: usize,
    quad_frag_shader: usize,
    quad_buffer: usize,
    pipeline_layout: wgpu::PipelineLayout,
    tex_layout: wgpu::BindGroupLayout,
    pipelines_by_texture: HashMap<usize, (wgpu::BindGroup, wgpu::RenderPipeline)>
}

impl BackgroundRender {
    pub fn new(init: &mut Initializer) -> Self {
        let tex_layout = init.simple_texture_bg_layout("quad_layout");

        Self {
            quad_vert_shader: init.load_shader_from_include(
                ShaderType::Vertex,
                wgpu::include_spirv!("../../../resources/shaders/quad_tex.vert.spv")
            ),
            quad_frag_shader: init.load_shader_from_include(
                ShaderType::Vertex,
                wgpu::include_spirv!("../../../resources/shaders/quad_tex.frag.spv")
            ),
            quad_buffer: init.make_buffer_with_data(
                &[2, 2],
                24,
                wgpu::BufferUsage::VERTEX,
                &[
                -1.0, 1.0, 0.0, 0.0, -1.0, -1.0, 0.0, 1.0, 1.0, -1.0, 1.0, 1.0, -1.0, 1.0, 0.0, 0.0,
                1.0, -1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.0,
                ]
            ),
            pipeline_layout: init.pipeline_layout(&[&tex_layout], "bgrender"),
            tex_layout,
            pipelines_by_texture: HashMap::new()
        }
    }

    fn get_bg_pipeline(&mut self, tex_id: usize, core: &mut Core) -> &(wgpu::BindGroup, wgpu::RenderPipeline) {
        if self.pipelines_by_texture.contains_key(&tex_id) {
            &self.pipelines_by_texture[&tex_id]
        } else {
            let quad_bg = core.simple_texture_bg(&self.tex_layout, tex_id);
            let pipeline = core.render_pipeline_simple(
                "QuadPipeline",
                &self.pipeline_layout,
                self.quad_vert_shader,
                self.quad_frag_shader,
                self.quad_buffer
            );
            self.pipelines_by_texture.insert(tex_id, (quad_bg, pipeline));
            self.get_bg_pipeline(tex_id, core)
        }
    }

    pub fn render(&mut self, device: &wgpu::Device, tex_id: usize) {
        let buffer_id = self.quad_buffer;
        let (quad_bg, pipeline) = self.get_bg_pipeline(tex_id, core);

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                    attachment: &core.frame.output.view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: true,
                    }
                }],
                depth_stencil_attachment: None,
            });
            rpass.set_pipeline(&pipeline);
            rpass.set_bind_group(0, &quad_bg, &[]);
            rpass.set_vertex_buffer(
                0, 
                core.buffers.get_buffer(buffer_id).buffer.as_ref().unwrap().slice(..)
            );
            rpass.draw(0..24, 0..1);
        }
        core.queue.submit(Some(encoder.finish()));
    }
}
