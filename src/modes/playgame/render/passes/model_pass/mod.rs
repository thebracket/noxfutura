use crate::modes::playgame::{Camera, CameraUniform, Model, Palette};
use bengine::*;

pub struct ModelsPass {
    pipeline: gpu::RenderPipeline,
    bind_group: gpu::BindGroup,
    palette_bind_group: gpu::BindGroup,
    tree_model: Model,
}

impl ModelsPass {
    pub fn new(palette: &Palette, tree_model: Model, uniforms: &CameraUniform) -> Self {
        let (terrain_vert, terrain_frag) = helpers::shader_from_bytes(
            bengine::gpu::include_spirv!("models.vert.spv"),
            bengine::gpu::include_spirv!("models.frag.spv"),
        );

        let dl = RENDER_CONTEXT.read();
        let ctx = dl.as_ref().unwrap();

        let buffer_template = FloatBuffer::<f32>::new(
            &[3, 3, 1],
            1,
            gpu::BufferUsage::VERTEX | gpu::BufferUsage::COPY_DST,
        );

        let bind_group_layout =
            ctx.device
                .create_bind_group_layout(&gpu::BindGroupLayoutDescriptor {
                    label: None,
                    entries: &[gpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: gpu::ShaderStage::VERTEX,
                        ty: gpu::BindingType::UniformBuffer {
                            dynamic: false,
                            min_binding_size: gpu::BufferSize::new(64),
                        },
                        count: None,
                    }],
                });

        let bind_group = ctx.device.create_bind_group(&gpu::BindGroupDescriptor {
            label: None,
            layout: &bind_group_layout,
            entries: &[gpu::BindGroupEntry {
                binding: 0,
                resource: gpu::BindingResource::Buffer(uniforms.uniform_buffer.slice(..)),
            }],
        });
        let palette_bind_group = ctx.device.create_bind_group(&gpu::BindGroupDescriptor {
            label: None,
            layout: &palette.bind_group_layout,
            entries: &[gpu::BindGroupEntry {
                binding: 0,
                resource: gpu::BindingResource::Buffer(palette.palette_buf.slice(..)),
            }],
        });
        let pipeline_layout = ctx
            .device
            .create_pipeline_layout(&gpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[&bind_group_layout, &palette.bind_group_layout],
                push_constant_ranges: &[],
            });
        let pipeline = ctx
            .device
            .create_render_pipeline(&gpu::RenderPipelineDescriptor {
                label: None,
                layout: Some(&pipeline_layout),
                vertex_stage: gpu::ProgrammableStageDescriptor {
                    module: SHADERS.read().get_module(terrain_vert),
                    entry_point: "main",
                },
                fragment_stage: Some(gpu::ProgrammableStageDescriptor {
                    module: SHADERS.read().get_module(terrain_frag),
                    entry_point: "main",
                }),
                rasterization_state: Some(gpu::RasterizationStateDescriptor {
                    front_face: gpu::FrontFace::Ccw,
                    cull_mode: gpu::CullMode::None,
                    ..Default::default()
                }),
                primitive_topology: gpu::PrimitiveTopology::TriangleList,
                color_states: &[gpu::ColorStateDescriptor {
                    format: ctx.swapchain_format,
                    color_blend: gpu::BlendDescriptor::REPLACE,
                    alpha_blend: gpu::BlendDescriptor::REPLACE,
                    write_mask: gpu::ColorWrite::ALL,
                }],
                depth_stencil_state: Some(gpu::DepthStencilStateDescriptor {
                    format: gpu::TextureFormat::Depth32Float,
                    depth_write_enabled: true,
                    depth_compare: gpu::CompareFunction::Less,
                    stencil: gpu::StencilStateDescriptor::default(),
                }),
                vertex_state: gpu::VertexStateDescriptor {
                    index_format: gpu::IndexFormat::Uint16,
                    vertex_buffers: &[buffer_template.descriptor()],
                },
                sample_count: 1,
                sample_mask: !0,
                alpha_to_coverage_enabled: false,
            });

        Self {
            bind_group,
            palette_bind_group,
            pipeline,
            tree_model,
        }
    }

    pub fn render(&self, core: &Core) {
        let dl = RENDER_CONTEXT.read();
        let ctx = dl.as_ref().unwrap();
        let tlock = TEXTURES.read();

        let mut encoder = ctx
            .device
            .create_command_encoder(&gpu::CommandEncoderDescriptor { label: None });

        {
            let mut rpass = encoder.begin_render_pass(&gpu::RenderPassDescriptor {
                color_attachments: &[gpu::RenderPassColorAttachmentDescriptor {
                    attachment: &core.frame.output.view,
                    resolve_target: None,
                    ops: gpu::Operations {
                        load: gpu::LoadOp::Load,
                        store: true,
                    },
                }],
                depth_stencil_attachment: Some(gpu::RenderPassDepthStencilAttachmentDescriptor {
                    attachment: tlock.get_view(0),
                    depth_ops: Some(gpu::Operations {
                        load: gpu::LoadOp::Load,
                        store: true,
                    }),
                    stencil_ops: None,
                }),
            });

            rpass.set_pipeline(&self.pipeline);
            rpass.set_bind_group(0, &self.bind_group, &[]);
            rpass.set_bind_group(1, &self.palette_bind_group, &[]);

            rpass.set_vertex_buffer(0, self.tree_model.vertex_buffer.slice());
            rpass.set_index_buffer(self.tree_model.index_buffer.slice(..));
            rpass.draw_indexed(0..self.tree_model.index_length, 0, 0..1);

            // Draw the model here
            /*for chunk in chunks.visible_chunks() {
                let buffer = chunk.maybe_render_chunk(camera_z);
                if let Some(buffer) = buffer {
                    rpass.set_vertex_buffer(0, buffer.0.slice());
                    rpass.draw(0..buffer.1, 0..1);
                }
            }*/
        }
        ctx.queue.submit(Some(encoder.finish()));
    }
}
