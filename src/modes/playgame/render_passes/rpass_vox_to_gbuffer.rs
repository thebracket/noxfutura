use crate::modes::playgame::chunks::Chunks;
use crate::engine::{Context, VertexBuffer};
use super::{camera::Camera, uniforms::Uniforms, gbuffer::GBuffer, texarray::TextureArray};
use super::super::VoxBuffer;
use legion::prelude::*;

pub struct VoxRenderPass {
    pub vox_models: VoxBuffer,
    pub shader_id: usize,
    pub render_pipeline: wgpu::RenderPipeline,
}

impl VoxRenderPass {
    pub fn new(
        context: &mut Context, 
        uniform_bind_group_layout: &wgpu::BindGroupLayout
    ) -> Self {
        let mut vox_models = VoxBuffer::new();
        vox_models.load(context);

        // Initialize camera and uniforms
        let camera = Camera::new(context.size.width, context.size.height);
        let mut uniforms = Uniforms::new();
        uniforms.update_view_proj(&camera);

        // Shader
        let shader_id = context.register_shader(
            "resources/shaders/voxmod.vert",
            "resources/shaders/voxmod.frag",
        );

        // WGPU Details
        let pipeline_layout =
            context
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    bind_group_layouts: &[&uniform_bind_group_layout],
                });
        let render_pipeline =
            context
                .device
                .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    layout: &pipeline_layout,
                    vertex_stage: wgpu::ProgrammableStageDescriptor {
                        module: &context.shaders[shader_id].vs_module,
                        entry_point: "main",
                    },
                    fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
                        module: &context.shaders[shader_id].fs_module,
                        entry_point: "main",
                    }),
                    rasterization_state: Some(wgpu::RasterizationStateDescriptor {
                        front_face: wgpu::FrontFace::Ccw,
                        cull_mode: wgpu::CullMode::Back,
                        depth_bias: 0,
                        depth_bias_slope_scale: 0.0,
                        depth_bias_clamp: 0.0,
                    }),
                    primitive_topology: wgpu::PrimitiveTopology::TriangleList,
                    color_states: &vec![
                        wgpu::ColorStateDescriptor {
                            format: context.swapchain_format,
                            color_blend: wgpu::BlendDescriptor::REPLACE,
                            alpha_blend: wgpu::BlendDescriptor::REPLACE,
                            write_mask: wgpu::ColorWrite::ALL,
                        },
                        wgpu::ColorStateDescriptor {
                            format: context.swapchain_format,
                            color_blend: wgpu::BlendDescriptor::REPLACE,
                            alpha_blend: wgpu::BlendDescriptor::REPLACE,
                            write_mask: wgpu::ColorWrite::ALL,
                        },
                        wgpu::ColorStateDescriptor {
                            format: context.swapchain_format,
                            color_blend: wgpu::BlendDescriptor::REPLACE,
                            alpha_blend: wgpu::BlendDescriptor::REPLACE,
                            write_mask: wgpu::ColorWrite::ALL,
                        },
                        wgpu::ColorStateDescriptor {
                            format: context.swapchain_format,
                            color_blend: wgpu::BlendDescriptor::REPLACE,
                            alpha_blend: wgpu::BlendDescriptor::REPLACE,
                            write_mask: wgpu::ColorWrite::ALL,
                        }
                    ],
                    depth_stencil_state: Some(wgpu::DepthStencilStateDescriptor {
                        format: crate::engine::texture::Texture::DEPTH_FORMAT,
                        depth_write_enabled: true,
                        depth_compare: wgpu::CompareFunction::Less,
                        stencil_front: wgpu::StencilStateFaceDescriptor::IGNORE,
                        stencil_back: wgpu::StencilStateFaceDescriptor::IGNORE,
                        stencil_read_mask: 0,
                        stencil_write_mask: 0,
                    }),
                    vertex_state: wgpu::VertexStateDescriptor {
                        index_format: wgpu::IndexFormat::Uint16,
                        vertex_buffers: &[vox_models.vertices.descriptor()],
                    },
                    sample_count: 1,
                    sample_mask: !0,
                    alpha_to_coverage_enabled: false,
                });

        // Build the result
        let builder = Self {
            shader_id,
            render_pipeline,
            vox_models,
        };
        builder
    }

    pub fn render(
        &mut self,
        context: &mut Context,
        depth_id: usize,
        frame: &wgpu::SwapChainOutput,
        gbuffer: &GBuffer,
        uniform_bg: &wgpu::BindGroup,
        camera_z: usize,
        ecs: &World
    ) {
        // Instances builder
        use crate::components::*;
        let mut vox_instances = Vec::new();
        let query = <(Read<Position>, Read<VoxelModel>)>::query();
        for (pos, vm) in query.iter(&ecs) {
            let first = self.vox_models.offsets[vm.index].0;
            let last = self.vox_models.offsets[vm.index].1;
            vox_instances.push((first, last-first));
        }

        // Render code
        let mut encoder = context
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[
                    wgpu::RenderPassColorAttachmentDescriptor {
                        attachment: &gbuffer.albedo.view,
                        resolve_target: None,
                        load_op: wgpu::LoadOp::Load,
                        store_op: wgpu::StoreOp::Store,
                        clear_color: wgpu::Color::BLUE,
                    },
                    wgpu::RenderPassColorAttachmentDescriptor {
                        attachment: &gbuffer.normal.view,
                        resolve_target: None,
                        load_op: wgpu::LoadOp::Load,
                        store_op: wgpu::StoreOp::Store,
                        clear_color: wgpu::Color::RED,
                    },
                    wgpu::RenderPassColorAttachmentDescriptor {
                        attachment: &gbuffer.pbr.view,
                        resolve_target: None,
                        load_op: wgpu::LoadOp::Load,
                        store_op: wgpu::StoreOp::Store,
                        clear_color: wgpu::Color::RED,
                    },
                    wgpu::RenderPassColorAttachmentDescriptor {
                        attachment: &gbuffer.coords.view,
                        resolve_target: None,
                        load_op: wgpu::LoadOp::Load,
                        store_op: wgpu::StoreOp::Store,
                        clear_color: wgpu::Color::RED,
                    }
                ],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachmentDescriptor {
                    attachment: &context.textures[depth_id].view,
                    depth_load_op: wgpu::LoadOp::Load,
                    depth_store_op: wgpu::StoreOp::Store,
                    clear_depth: 1.0,
                    stencil_load_op: wgpu::LoadOp::Clear,
                    stencil_store_op: wgpu::StoreOp::Store,
                    clear_stencil: 0,
                }),
            });

            rpass.set_pipeline(&self.render_pipeline);
            rpass.set_bind_group(0, &uniform_bg, &[]);
            rpass.set_vertex_buffer(0, &self.vox_models.vertices.buffer.as_ref().unwrap(), 0, 0);

            // Render
            for i in vox_instances.iter() {
                rpass.draw(i.0 .. i.1, 0..1);
            }
        }
        context.queue.submit(&[encoder.finish()]);
    }
}
