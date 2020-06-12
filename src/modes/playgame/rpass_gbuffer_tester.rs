use crate::engine::{Context, VertexBuffer};

pub struct GBufferTestPass {
    pub vb: VertexBuffer<f32>,
    pub shader_id: usize,
    pub render_pipeline: wgpu::RenderPipeline,
    pub bind_group : wgpu::BindGroup
}

impl GBufferTestPass {
    pub fn new(context: &mut Context, gbuffer: &super::gbuffer::GBuffer) -> Self {
        // Simple quad VB for output
        let mut vb = VertexBuffer::<f32>::new(&[2, 2]);
        vb.add_slice(&[
            -1.0, 1.0, 0.0, 0.0, -1.0, -1.0, 0.0, 1.0, 1.0, -1.0, 1.0, 1.0, -1.0, 1.0, 0.0, 0.0,
            1.0, -1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.0,
        ]);
        vb.build(&context.device, wgpu::BufferUsage::VERTEX);

        // Shader
        let shader_id = context.register_shader(
            "resources/shaders/gbuffer_test.vert",
            "resources/shaders/gbuffer_test.frag",
        );

        // Bind Group Layout
        let bind_group_layout =
            context
                .device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    bindings: &[
                        wgpu::BindGroupLayoutEntry {
                            binding: 0,
                            visibility: wgpu::ShaderStage::FRAGMENT,
                            ty: wgpu::BindingType::SampledTexture {
                                multisampled: false,
                                dimension: wgpu::TextureViewDimension::D2,
                                component_type: wgpu::TextureComponentType::Uint,
                            },
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 1,
                            visibility: wgpu::ShaderStage::FRAGMENT,
                            ty: wgpu::BindingType::Sampler { comparison: false },
                        },
                    ],
                    label: None,
                }
            );
        let bind_group = context
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &bind_group_layout,
                bindings: &[
                    wgpu::Binding {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&gbuffer.normal.view),
                    },
                    wgpu::Binding {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&gbuffer.normal.sampler),
                    },
                ],
                label: None,
            }
        );

        // WGPU Details
        let pipeline_layout =
            context
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    bind_group_layouts: &[&bind_group_layout],
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
                    ],
                    depth_stencil_state: None,
                    vertex_state: wgpu::VertexStateDescriptor {
                        index_format: wgpu::IndexFormat::Uint16,
                        vertex_buffers: &[vb.descriptor()],
                    },
                    sample_count: 1,
                    sample_mask: !0,
                    alpha_to_coverage_enabled: false,
                }
            );

        Self{
            vb,
            shader_id,
            render_pipeline,
            bind_group
        }
    }

    pub fn render(
        &mut self,
        context: &mut Context,
        frame: &wgpu::SwapChainOutput
    ) {
        let mut encoder = context
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[
                    wgpu::RenderPassColorAttachmentDescriptor {
                        attachment: &frame.view,
                        resolve_target: None,
                        load_op: wgpu::LoadOp::Clear,
                        store_op: wgpu::StoreOp::Store,
                        clear_color: wgpu::Color::BLUE,
                    },
                ],
                depth_stencil_attachment: None
            });

            rpass.set_pipeline(&self.render_pipeline);
            rpass.set_bind_group(0, &self.bind_group, &[]);

            if self.vb.len() > 0 {
                rpass.set_vertex_buffer(0, &self.vb.buffer.as_ref().unwrap(), 0, 0);
                rpass.draw(0..self.vb.len(), 0..1);
            }
        }
        context.queue.submit(&[encoder.finish()]);
    }
}