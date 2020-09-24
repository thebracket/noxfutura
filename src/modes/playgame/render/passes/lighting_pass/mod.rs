use bengine::*;
use legion::*;

use crate::modes::playgame::GBuffer;

pub struct LightingPass {
    vb: FloatBuffer<f32>,
    pipeline: gpu::RenderPipeline,
    bind_group: gpu::BindGroup,
}

impl LightingPass {
    pub fn new(gbuffer: &GBuffer) -> Self {
        // Shader
        let (light_vert, light_frag) = helpers::shader_from_bytes(
            bengine::gpu::include_spirv!("lighting.vert.spv"),
            bengine::gpu::include_spirv!("lighting.frag.spv"),
        );

        // Simple quad VB for output
        let mut vb = FloatBuffer::<f32>::new(&[2, 2],24, gpu::BufferUsage::VERTEX);
        vb.add_slice(&[
            -1.0, 1.0, 0.0, 0.0, -1.0, -1.0, 0.0, 1.0, 1.0, -1.0, 1.0, 1.0, -1.0, 1.0, 0.0, 0.0,
            1.0, -1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.0,
        ]);
        vb.build();

        // Pipeline setup
        let dl = RENDER_CONTEXT.read();
        let ctx = dl.as_ref().unwrap();

        let bind_group_layout =
            ctx.device
                .create_bind_group_layout(&gpu::BindGroupLayoutDescriptor {
                    label: None,
                    entries: &[
                        gpu::BindGroupLayoutEntry {
                            binding: 0,
                            visibility: gpu::ShaderStage::FRAGMENT,
                            ty: gpu::BindingType::SampledTexture {
                                multisampled: false,
                                dimension: gpu::TextureViewDimension::D2,
                                component_type: gpu::TextureComponentType::Uint,
                            },
                            count: None,
                        },
                        gpu::BindGroupLayoutEntry {
                            binding: 1,
                            visibility: gpu::ShaderStage::FRAGMENT,
                            ty: gpu::BindingType::Sampler { comparison: true },
                            count: None,
                        },

                        gpu::BindGroupLayoutEntry {
                            binding: 2,
                            visibility: gpu::ShaderStage::FRAGMENT,
                            ty: gpu::BindingType::SampledTexture {
                                multisampled: false,
                                dimension: gpu::TextureViewDimension::D2,
                                component_type: gpu::TextureComponentType::Uint,
                            },
                            count: None,
                        },
                        gpu::BindGroupLayoutEntry {
                            binding: 3,
                            visibility: gpu::ShaderStage::FRAGMENT,
                            ty: gpu::BindingType::Sampler { comparison: true },
                            count: None,
                        },
                    ],
                }
            );

        let bind_group = ctx.device.create_bind_group(&gpu::BindGroupDescriptor {
            label: None,
            layout: &bind_group_layout,
            entries: &[
                gpu::BindGroupEntry {
                    binding: 0,
                    resource: gpu::BindingResource::TextureView(&gbuffer.albedo.view),
                },
                gpu::BindGroupEntry {
                    binding: 1,
                    resource: gpu::BindingResource::Sampler(&gbuffer.albedo.sampler),
                },
                gpu::BindGroupEntry {
                    binding: 2,
                    resource: gpu::BindingResource::TextureView(&gbuffer.normal.view),
                },
                gpu::BindGroupEntry {
                    binding: 3,
                    resource: gpu::BindingResource::Sampler(&gbuffer.normal.sampler),
                },
            ],
        });

        let pipeline_layout = ctx
            .device
            .create_pipeline_layout(&gpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[&bind_group_layout],
                push_constant_ranges: &[],
            });
        let pipeline = ctx
            .device
            .create_render_pipeline(&gpu::RenderPipelineDescriptor {
                label: None,
                layout: Some(&pipeline_layout),
                vertex_stage: gpu::ProgrammableStageDescriptor {
                    module: SHADERS.read().get_module(light_vert),
                    entry_point: "main",
                },
                fragment_stage: Some(gpu::ProgrammableStageDescriptor {
                    module: SHADERS.read().get_module(light_frag),
                    entry_point: "main",
                }),
                rasterization_state: Some(gpu::RasterizationStateDescriptor {
                    front_face: gpu::FrontFace::Ccw,
                    cull_mode: gpu::CullMode::None,
                    ..Default::default()
                }),
                primitive_topology: gpu::PrimitiveTopology::TriangleList,
                color_states: &[
                    gpu::ColorStateDescriptor {
                        format: ctx.swapchain_format,
                        color_blend: gpu::BlendDescriptor::REPLACE,
                        alpha_blend: gpu::BlendDescriptor::REPLACE,
                        write_mask: gpu::ColorWrite::ALL,
                    },
                ],
                depth_stencil_state: None,
                vertex_state: gpu::VertexStateDescriptor {
                    index_format: gpu::IndexFormat::Uint16,
                    vertex_buffers: &[vb.descriptor()],
                },
                sample_count: 1,
                sample_mask: !0,
                alpha_to_coverage_enabled: false,
            }
        );

        Self{
            vb,
            bind_group,
            pipeline
        }
    }

    pub fn render(&mut self, core: &Core, ecs: &mut World) {
        let dl = RENDER_CONTEXT.read();
        let ctx = dl.as_ref().unwrap();

        let mut encoder = ctx
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
            rpass.set_bind_group(0, &self.bind_group, &[]);

            rpass.set_vertex_buffer(0, self.vb.slice());
            rpass.draw(0..self.vb.len(), 0..1);
        }
        ctx.queue.submit(Some(encoder.finish()));
    }
}